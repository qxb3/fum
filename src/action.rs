use mpris::{LoopStatus, Player};
use serde::Deserialize;

use crate::{fum::Fum, FumResult};

macro_rules! if_player {
    ($player:expr, $callback:expr) => {
        if let Some(player) = $player {
            $callback(player)?;
        }
    };
}

#[derive(Debug, Clone)]
pub enum Action {
    Quit,

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
    LoopPlaylist,
    LoopTrack,
    LoopCycle
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let action_str: &str = Deserialize::deserialize(deserializer)?;

        match action_str {
            "quit()"            => Ok(Action::Quit),

            "stop()"            => Ok(Action::Stop),
            "play()"            => Ok(Action::Play),
            "pause()"           => Ok(Action::Pause),

            "prev()"            => Ok(Action::Prev),
            "play_pause()"      => Ok(Action::PlayPause),
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
        match action {
            Action::Quit            => fum.exit = true,

            Action::Stop            => if_player!(&fum.player, |player: &Player| player.stop()),
            Action::Play            => if_player!(&fum.player, |player: &Player| player.play()),
            Action::Pause           => if_player!(&fum.player, |player: &Player| player.pause()),

            Action::Prev            => if_player!(&fum.player, |player: &Player| player.previous()),
            Action::PlayPause       => if_player!(&fum.player, |player: &Player| player.play_pause()),
            Action::Next            => if_player!(&fum.player, |player: &Player| player.next()),

            Action::ShuffleOff      => if_player!(&fum.player, |player: &Player| player.set_shuffle(true)),
            Action::ShuffleToggle   => if_player!(&fum.player, |player: &Player| player.set_shuffle(!player.get_shuffle()?)),
            Action::ShuffleOn       => if_player!(&fum.player, |player: &Player| player.set_shuffle(false)),

            Action::LoopNone        => if_player!(&fum.player, |player: &Player| player.set_loop_status(LoopStatus::None)),
            Action::LoopPlaylist    => if_player!(&fum.player, |player: &Player| player.set_loop_status(LoopStatus::Playlist)),
            Action::LoopTrack       => if_player!(&fum.player, |player: &Player| player.set_loop_status(LoopStatus::Track)),
            Action::LoopCycle       => {
                if let Some(player) = &fum.player {
                    let loop_status = player.get_loop_status()?;

                    match loop_status {
                        LoopStatus::None        => player.set_loop_status(LoopStatus::Playlist)?,
                        LoopStatus::Playlist    => player.set_loop_status(LoopStatus::Track)?,
                        LoopStatus::Track       => player.set_loop_status(LoopStatus::None)?
                    }
                }
            }
        }

        Ok(())
    }
}
