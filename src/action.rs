use mpris::{LoopStatus, Player};

use crate::fum::FumResult;

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
    pub fn run_str(action: &str, player: &Option<Player>) -> FumResult<()> {
        match action {
            "stop()"    => Action::run(Action::Stop, player)?,
            "play()"    => Action::run(Action::Play, player)?,
            "pause()"   => Action::run(Action::Pause, player)?,

            "prev()"        => Action::run(Action::Prev, player)?,
            "play_pause()"  => Action::run(Action::PlayPause, player)?,
            "next()"        => Action::run(Action::Next, player)?,

            "shuffle_off()"     => Action::run(Action::ShuffleOff, player)?,
            "shuffle_toggle()"  => Action::run(Action::ShuffleToggle, player)?,
            "shuffle_on()"      => Action::run(Action::ShuffleOn, player)?,

            "loop_none()"       => Action::run(Action::LoopNone, player)?,
            "loop_track()"      => Action::run(Action::LoopTrack, player)?,
            "loop_playlist()"   => Action::run(Action::LoopPlaylist, player)?,
            "loop_cycle()"   => Action::run(Action::LoopCycle, player)?,

            _ => {}
        }

        Ok(())
    }

    pub fn run(action: Action, player: &Option<Player>) -> FumResult<()> {
        if let Some(_player) = player {
            match action {
                Action::Stop => _player.stop()?,
                Action::Play => _player.play()?,
                Action::Pause => _player.pause()?,

                Action::Prev => _player.previous()?,
                Action::PlayPause => _player.play_pause()?,
                Action::Next => _player.next()?,

                Action::ShuffleOff => _player.set_shuffle(false)?,
                Action::ShuffleToggle => _player.set_shuffle(!_player.get_shuffle()?)?,
                Action::ShuffleOn => _player.set_shuffle(true)?,

                Action::LoopNone => _player.set_loop_status(LoopStatus::None)?,
                Action::LoopPlaylist => _player.set_loop_status(LoopStatus::Playlist)?,
                Action::LoopTrack => _player.set_loop_status(LoopStatus::Track)?,
                Action::LoopCycle => {
                    let loop_state = _player.get_loop_status()?;

                    match loop_state {
                        LoopStatus::None => Action::run(Action::LoopPlaylist, player)?,
                        LoopStatus::Playlist => Action::run(Action::LoopTrack, player)?,
                        LoopStatus::Track => Action::run(Action::LoopNone, player)?,
                    }
                },
            }
        }

        Ok(())
    }
}
