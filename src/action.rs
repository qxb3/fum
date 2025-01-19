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
    LoopPlaylist,
    LoopCycle
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
            "loop_cycle()"   => Action::run(Action::LoopCycle, player),

            _ => {}
        }
    }

    pub fn run(action: Action, player: &Option<Player>) {
        if let Some(_player) = player {
            match action {
                Action::Stop => _player.stop().expect("Failed to stop player"),
                Action::Play => _player.play().expect("Failed to play player"),
                Action::Pause => _player.pause().expect("Failed to pause player"),

                Action::Prev => _player.previous().expect("Failed to prev player"),
                Action::PlayPause => _player.play_pause().expect("Failed to play/pause player"),
                Action::Next => _player.next().expect("Failed to next player"),

                Action::ShuffleOff => _player.set_shuffle(false).expect("Failed to unshuffle"),
                Action::ShuffleToggle => _player.set_shuffle(!_player.get_shuffle().expect("Failed to get shuffle state")).expect("Failed to toggle shuffle"),
                Action::ShuffleOn => _player.set_shuffle(true).expect("Failed to shuffle"),

                Action::LoopNone => _player.set_loop_status(LoopStatus::None).expect("Failed to set loop to none"),
                Action::LoopPlaylist => _player.set_loop_status(LoopStatus::Playlist).expect("Failed to set loop to playlist"),
                Action::LoopTrack => _player.set_loop_status(LoopStatus::Track).expect("Failed to set loop to track"),
                Action::LoopCycle => {
                    let loop_state = _player.get_loop_status().expect("Failed to get loop state");

                    match loop_state {
                        LoopStatus::None => Action::run(Action::LoopPlaylist, player),
                        LoopStatus::Playlist => Action::run(Action::LoopTrack, player),
                        LoopStatus::Track => Action::run(Action::LoopNone, player),
                    }
                },
            }
        }
    }
}
