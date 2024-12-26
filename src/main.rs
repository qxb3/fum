mod utils;

use image::DynamicImage;
use mpris::PlaybackStatus;
use ratatui::{
    crossterm::{
        event::{
            self, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent,
            MouseEventKind,
        },
        execute,
    },
    layout::{Constraint, Flex, Layout, Position, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui_image::{picker::Picker, StatefulImage};
use std::{
    io::stdout,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

const WIDTH: u16 = 20;
const HEIGHT: u16 = 15;

struct Buttons {
    prev: Rect,
    toggle: Rect,
    next: Rect
}

#[derive(Debug)]
struct Meta {
    title: String,
    artists: Vec<String>,
    status: PlaybackStatus,
    length: Duration,
    cover_art: DynamicImage
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            title: "No Music".to_string(),
            artists: vec!["Artist".to_string()],
            status: PlaybackStatus::Stopped,
            length: Duration::from_secs(0),
            cover_art: DynamicImage::default()
        }
    }
}

enum Message {
    Tick,

    MetaChanged(Meta),

    Prev,
    Toggle,
    Next,

    LeftClick {
        col: u16,
        row: u16
    },

    Err(String),
    #[allow(unused)]
    Dbg(String),

    Exit
}

fn main() {
    execute!(stdout(), EnableMouseCapture)
        .expect("Failed to enable mouse capture.");

    let mut terminal = ratatui::init();
    let picker = Picker::from_query_stdio()
        .expect("Failed to query font size. This terminal might not be supported.");

    let mut buttons = Buttons {
        prev: Rect::default(),
        toggle: Rect::default(),
        next: Rect::default(),
    };

    let mut meta = Meta::default();
    let mut player = utils::player::get_active_player();

    let (tx, rx) = mpsc::channel::<Message>();

    tick(tx.clone());
    handle_mpris_events(tx.clone());
    handle_term_events(tx.clone());

    loop {
        let event = rx.recv()
            .expect("Failed to receive event");

        match event {
            Message::Tick => {
                terminal
                    .draw(|frame| draw(
                        frame,
                        picker,
                        &mut buttons,
                        &meta
                    )).expect("Failed to draw frame.");
            },
            Message::MetaChanged(Meta { title, artists, status, length, cover_art }) => {
                player = utils::player::get_active_player();

                meta.title = title;
                meta.artists = artists;
                meta.status = status;
                meta.length = length;
                meta.cover_art = cover_art;
            },
            Message::LeftClick { col, row } => {
                match (col, row) {
                    (x, y) if buttons.prev.contains(Position { x, y })      => send_message!(tx, Message::Prev),
                    (x, y) if buttons.toggle.contains(Position { x, y })    => send_message!(tx, Message::Toggle),
                    (x, y) if buttons.next.contains(Position { x, y })      => send_message!(tx, Message::Next),
                    _ => {}
                }
            },
            Message::Prev => {
                if let Ok(player) = &player {
                    player.previous()
                        .expect("Cannot prev.");
                }
           },
            Message::Toggle => {
                if let Ok(player) = &player {
                    player.play_pause()
                        .expect("Cannot toggle.");
                }
            },
            Message::Next => {
                if let Ok(player) = &player {
                    player.next()
                        .expect("Cannot next.");
                }
            },

            Message::Err(err) => {
                eprintln!("[ERROR] {}", err);
                utils::restore();
                break;
            },
            Message::Exit => {
                utils::restore();
                break;
            },
            Message::Dbg(message) => println!("[DEBUG] {}", message)
        }
    }
}

fn draw(
    frame: &mut Frame<'_>,
    picker: Picker,
    buttons: &mut Buttons,
    meta: &Meta
) {
    let [area] = Layout::horizontal([Constraint::Length(WIDTH)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [area] = Layout::vertical([Constraint::Length(HEIGHT)])
        .flex(Flex::Center)
        .areas(area);

    // Terminal window is too small
    if frame.area().width < WIDTH ||
        frame.area().height < HEIGHT {
        frame.render_widget(
            Paragraph::new(format!(
                "Terminal window is too small. Must have atleast ({}x{}).",
                WIDTH, HEIGHT
            ))
                .wrap(Wrap::default())
                .centered()
                .block(Block::new().borders(Borders::ALL)),
            area
        );

        return;
    }

    let [top, bottom] = Layout::vertical([
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ]).areas(area);

    let [title, artist, _, controls, _, progress, pos] = Layout::vertical([
        Constraint::Length(1); 7
    ])
        .flex(Flex::Center)
        .areas(bottom);

    let [prev, toggle, next] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ])
        .flex(Flex::Center)
        .areas(controls);

    buttons.prev = prev;
    buttons.toggle = toggle;
    buttons.next = next;

    let [pos_current, _, length] = Layout::horizontal([
        Constraint::Length(4),
        Constraint::Fill(8),
        Constraint::Length(4),
    ])
        .spacing(1)
        .areas(pos);

    frame.render_stateful_widget(
        StatefulImage::default(),
        top,
        &mut picker.new_resize_protocol(meta.cover_art.clone())
    );

    frame.render_widget(
        Text::from(
            utils::truncate(&meta.title, 12).as_str()
        ).centered(),
        title
    );

    frame.render_widget(
        Text::from(
            utils::truncate(&meta.artists.join(", "), 12).as_str()
        ).centered(),
        artist
    );

    frame.render_widget(Text::from("󰒮").centered(), buttons.prev);
    frame.render_widget(
        Text::from(
            utils::player::get_status_icon(&meta.status)
        ).centered(),
        buttons.toggle
    );
    frame.render_widget(Text::from("󰒭").centered(), buttons.next);

    frame.render_widget(Text::from("󰝤".repeat(WIDTH.into())), progress);
    frame.render_widget(Text::from("0:32").left_aligned(), pos_current);

    frame.render_widget(
        Text::from(format!("{}:{:02}", meta.length.as_secs() / 60, meta.length.as_secs() % 60)).right_aligned(),
        length
    );
}

fn handle_mpris_events(tx: Sender<Message>) {
    thread::spawn(move || {
        loop {
            let player = match utils::player::get_active_player() {
                Ok(player) => player,
                Err(_) => {
                    send_message!(tx, Message::MetaChanged(Meta::default()));
                    continue;
                }
            };

            let meta = match utils::player::get_meta(&player) {
                Ok(meta) => meta,
                Err(err) => {
                    send_err!(tx, err.to_string());
                    continue;
                }
            };

            tx.send(Message::MetaChanged(Meta {
                title: meta.title,
                artists: meta.artists,
                status: meta.status,
                length: meta.length,
                cover_art: meta.cover_art
            })).unwrap();

            let events = match player.events() {
                Ok(events) => events,
                Err(err) => {
                    send_err!(tx, format!("Failed to start mpris event stream: {}", err));
                    return;
                }
            };

            for event in events {
                match event {
                    Ok(_) => {
                        let meta = match utils::player::get_meta(&player) {
                            Ok(meta) => meta,
                            Err(_) => break
                        };

                        send_message!(tx, Message::MetaChanged(Meta {
                            title: meta.title,
                            artists: meta.artists,
                            status: meta.status,
                            length: meta.length,
                            cover_art: meta.cover_art
                        }));
                    },
                    Err(_) => break
                }
            }
        }
    });
}

fn handle_key_events(keycode: KeyCode, tx: Sender<Message>) {
    match keycode {
        KeyCode::Char('q') => send_message!(tx, Message::Exit),
        KeyCode::Char('p') => send_message!(tx, Message::Prev),
        KeyCode::Char(' ') => send_message!(tx, Message::Toggle),
        KeyCode::Char('n') => send_message!(tx, Message::Next),
        _ => {}
    }
}

fn handle_mouse_events(mouse: MouseEvent, tx: Sender<Message>) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => send_message!(tx, Message::LeftClick { col: mouse.column, row: mouse.row }),
        _ => {}
    }
}

fn handle_term_events(tx: Sender<Message>) {
    thread::spawn(move || {
        loop {
            match event::read() {
                Ok(event) => match event {
                    Event::Key(key) if key.kind == KeyEventKind::Press  => handle_key_events(key.code, tx.clone()),
                    Event::Mouse(mouse)                                 => handle_mouse_events(mouse, tx.clone()),
                    _ => {}
                },
                Err(err) => send_err!(tx, err.to_string()),
            }
        }
    });
}

fn tick(tx: Sender<Message>) {
    thread::spawn(move || {
        loop {
            send_message!(tx, Message::Tick);
            thread::sleep(Duration::from_millis(100));
        }
    });
}
