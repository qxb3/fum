mod cli;
mod ui;
mod utils;
mod config;

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
    layout::Position
};
use config::Config;
use ui::Ui;
use std::{
    io::stdout, process, sync::mpsc::{self, Sender}, thread, time::{Duration, Instant}
};

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

#[derive(Debug)]
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
    let config = match cli::run() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[ERR] {}.", err);
            process::exit(1);
        }
    };

    execute!(stdout(), EnableMouseCapture)
        .expect("Failed to enable mouse capture.");

    let mut terminal = ratatui::init();

    let mut meta = Meta::default();
    let mut player = utils::player::get_player(&config);

    let mut ui = Ui::new(&config)
        .expect("Failed to create ui.");

    let (tx, rx) = mpsc::channel::<Message>();

    tick(tx.clone());
    handle_mpris_events(tx.clone(), config.clone());
    handle_term_events(tx.clone());

    let mut now = Instant::now();
    let mut current_progress = Duration::from_secs(0);

    if let Ok(player) = &player {
        current_progress = player
            .get_position()
            .expect("Failed to get player's position.");
    }

    loop {
        let event = rx.recv()
            .expect("Failed to receive event");

        match event {
            Message::Tick => {
                if now.elapsed() >= Duration::from_secs(1) {
                    if let Ok(player) = &player {
                        current_progress = player
                            .get_position()
                            .expect("Failed to get player's position.");
                    }

                    now = Instant::now();
                }

                terminal.draw(|frame| ui.draw(frame, &meta, &current_progress))
                    .expect("Failed to draw frame.");
            },
            Message::MetaChanged(Meta { title, artists, status, length, cover_art }) => {
                player = utils::player::get_player(&config);
                current_progress = Duration::from_secs(0);

                meta.title = title;
                meta.artists = artists;
                meta.status = status;
                meta.length = length;
                meta.cover_art = cover_art;
            },
            Message::LeftClick { col, row } => {
                match (col, row) {
                    (x, y) if ui.playback_buttons.prev.contains(Position { x, y })      => send_message!(tx, Message::Prev),
                    (x, y) if ui.playback_buttons.toggle.contains(Position { x, y })    => send_message!(tx, Message::Toggle),
                    (x, y) if ui.playback_buttons.next.contains(Position { x, y })      => send_message!(tx, Message::Next),
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
                println!("exited.");
                break;
            },
            Message::Dbg(message) => println!("[DEBUG] {}", message)
        }
    }
}

fn handle_mpris_events(tx: Sender<Message>, config: Config) {
    thread::spawn(move || {
        loop {
            let player = match utils::player::get_player(&config) {
                Ok(player) => player,
                Err(_) => {
                    send_message!(tx, Message::MetaChanged(Meta::default()));
                    continue;
                }
            };

            let meta = match utils::player::get_meta(&player) {
                Ok(meta) => meta,
                Err(_) => {
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
            thread::sleep(Duration::from_millis(250));
        }
    });
}
