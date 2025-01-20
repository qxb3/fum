use mpris::LoopStatus;
use serde::Deserialize;

use crate::fum::{Fum, FumResult};

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
            "play_pause()"       => Ok(Action::PlayPause),
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
    pub fn run(action: &Action, fum: &mut Fum) -> FumResult<()> {
        if let Some(player) = &fum.player {
            match action {
                Action::Stop            => player.stop()?,
                Action::Play            => player.play()?,
                Action::Pause           => player.pause()?,

                Action::Prev            => player.previous()?,
                Action::PlayPause       => player.play_pause()?,
                Action::Next            => player.next()?,

                Action::ShuffleOff      => player.set_shuffle(false)?,
                Action::ShuffleToggle   => player.set_shuffle(!player.get_shuffle()?)?,
                Action::ShuffleOn       => player.set_shuffle(true)?,

                Action::LoopNone        => player.set_loop_status(LoopStatus::None)?,
                Action::LoopPlaylist    => player.set_loop_status(LoopStatus::Playlist)?,
                Action::LoopTrack       => player.set_loop_status(LoopStatus::Track)?,
                Action::LoopCycle       => {
                    let loop_state = player.get_loop_status()?;

                    match loop_state {
                        LoopStatus::None        => Action::run(&Action::LoopPlaylist, fum)?,
                        LoopStatus::Playlist    => Action::run(&Action::LoopTrack, fum)?,
                        LoopStatus::Track       => Action::run(&Action::LoopNone, fum)?,
                    }
                },
            }
        }

        Ok(())
    }
}
