use mpris::{LoopStatus, Player};

pub enum Action {
    Stop,
    Play,
    Pause,

    Prev,
    PlayPause,
    Next,

    ShuffleOff,
    ShuffleToggle,
    ShuffleOn,

    LoopNone,
    LoopTrack,
    LoopPlaylist
}

impl Action {
    pub fn run_str(action: &str, player: &Option<Player>) {
        match action {
            "stop()"    => Action::run(Action::Stop, player),
            "play()"    => Action::run(Action::Play, player),
            "pause()"   => Action::run(Action::Pause, player),

            "prev()"        => Action::run(Action::Prev, player),
            "play_pause()"  => Action::run(Action::PlayPause, player),
            "next()"        => Action::run(Action::Next, player),

            "shuffle_off()"     => Action::run(Action::ShuffleOff, player),
            "shuffle_toggle()"  => Action::run(Action::ShuffleToggle, player),
            "shuffle_on()"      => Action::run(Action::ShuffleOn, player),

            "loop_none()"       => Action::run(Action::LoopNone, player),
            "loop_track()"      => Action::run(Action::LoopTrack, player),
            "loop_playlist()"   => Action::run(Action::LoopPlaylist, player),

            _ => {}
        }
    }

    pub fn run(action: Action, player: &Option<Player>) {
        if let Some(player) = player {
            match action {
                Action::Stop => player.stop().expect("Failed to stop player"),
                Action::Play => player.play().expect("Failed to play player"),
                Action::Pause => player.pause().expect("Failed to pause player"),

                Action::Prev => player.previous().expect("Failed to prev player"),
                Action::PlayPause => player.play_pause().expect("Failed to play/pause player"),
                Action::Next => player.next().expect("Failed to next player"),

                Action::ShuffleOff => player.set_shuffle(false).expect("Failed to unshuffle"),
                Action::ShuffleToggle => player.set_shuffle(!player.get_shuffle().expect("Failed to get shuffle state")).expect("Failed to toggle shuffle"),
                Action::ShuffleOn => player.set_shuffle(true).expect("Failed to shuffle"),

                Action::LoopNone => player.set_loop_status(LoopStatus::None).expect("Failed to set loop to none"),
                Action::LoopTrack => player.set_loop_status(LoopStatus::Track).expect("Failed to set loop to track"),
                Action::LoopPlaylist => player.set_loop_status(LoopStatus::Playlist).expect("Failed to set loop to playlist"),
            }
        }
    }
}
