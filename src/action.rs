use mpris::{LoopStatus, Player};
use serde::Deserialize;

use crate::fum::FumResult;

#[derive(Debug, Clone)]
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

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let action_str: &str = Deserialize::deserialize(deserializer)?;

        match action_str {
            "stop()"            => Ok(Action::Stop),
            "play()"            => Ok(Action::Play),
            "pause()"           => Ok(Action::Pause),

            "prev()"            => Ok(Action::Prev),
            "playpause()"       => Ok(Action::PlayPause),
            "next()"            => Ok(Action::Next),

            "shuffle_off()"     => Ok(Action::ShuffleOff),
            "shuffle_toggle()"  => Ok(Action::ShuffleToggle),
            "shuffle_on()"      => Ok(Action::ShuffleOn),

            "loop_none()"       => Ok(Action::LoopNone),
            "loop_track()"      => Ok(Action::LoopTrack),
            "loop_playlist()"   => Ok(Action::LoopPlaylist),
            "loop_cycle()"      => Ok(Action::LoopCycle),

            _ => Err(serde::de::Error::custom(format!("Unknown action: {}", action_str)))
        }
    }
}

impl Action {
    pub fn run(action: &Action, player: &Option<Player>) -> FumResult<()> {
        if let Some(_player) = player {
            match action {
                Action::Stop            => _player.stop()?,
                Action::Play            => _player.play()?,
                Action::Pause           => _player.pause()?,

                Action::Prev            => _player.previous()?,
                Action::PlayPause       => _player.play_pause()?,
                Action::Next            => _player.next()?,

                Action::ShuffleOff      => _player.set_shuffle(false)?,
                Action::ShuffleToggle   => _player.set_shuffle(!_player.get_shuffle()?)?,
                Action::ShuffleOn       => _player.set_shuffle(true)?,

                Action::LoopNone        => _player.set_loop_status(LoopStatus::None)?,
                Action::LoopPlaylist    => _player.set_loop_status(LoopStatus::Playlist)?,
                Action::LoopTrack       => _player.set_loop_status(LoopStatus::Track)?,
                Action::LoopCycle       => {
                    let loop_state = _player.get_loop_status()?;

                    match loop_state {
                        LoopStatus::None        => Action::run(&Action::LoopPlaylist, player)?,
                        LoopStatus::Playlist    => Action::run(&Action::LoopTrack, player)?,
                        LoopStatus::Track       => Action::run(&Action::LoopNone, player)?,
                    }
                },
            }
        }

        Ok(())
    }
}
