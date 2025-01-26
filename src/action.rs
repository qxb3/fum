use std::time::Duration;

use mpris::{LoopStatus, Player};
use regex::Regex;
use serde::{de, Deserialize};

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
    LoopCycle,

    Forward(i64),
    Backward(i64),
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let action_str: &str = Deserialize::deserialize(deserializer)?;

        let forward_re = Regex::new(r"forward\((-?\d+)\)").unwrap();
        let backward_re = Regex::new(r"backward\((-?\d+)\)").unwrap();

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

            a if forward_re.is_match(a) => {
                if let Some(captures) = forward_re.captures(a) {
                    match captures[1].parse::<i64>() {
                        Ok(offset) => return Ok(Action::Forward(offset)),
                        Err(_) => return Err(de::Error::custom("Invalid forward() offset format"))
                    }
                }

                Err(de::Error::custom("Invalid forward() format"))
            },

            a if backward_re.is_match(a) => {
                if let Some(captures) = backward_re.captures(a) {
                    match captures[1].parse::<i64>() {
                        Ok(offset) => return Ok(Action::Backward(offset)),
                        Err(_) => return Err(de::Error::custom("Invalid backward() offset format"))
                    }
                }

                Err(de::Error::custom("Invalid backward() format"))
            },

            // Error if forward() / backward() has no value inside
            "forward()" => Err(de::Error::custom(format!("Invalid forward() format, needs value inside"))),
            "backward()" => Err(de::Error::custom(format!("Invalid backward() format, needs value inside"))),

            _ => Err(de::Error::custom(format!("Unknown action: {}", action_str)))
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
            },

            Action::Forward(offset)     => if_player!(&fum.player, |player: &Player| {
                fum.redraw = true;

                if let Some(track_id) = &fum.state.meta.track_id {
                    match offset {
                        -1  => return player.set_position(track_id.clone(), &fum.state.meta.length),
                        _   => return player.seek_forwards(&Duration::from_millis(*offset as u64))
                    }
                }

                unreachable!()
            }),
            Action::Backward(offset)     => if_player!(&fum.player, |player: &Player| {
                fum.redraw = true;

                if let Some(track_id) = &fum.state.meta.track_id {
                    match offset {
                        -1   => return player.set_position(track_id.clone(), &Duration::from_secs(0)),
                        _   => return player.seek_backwards(&Duration::from_millis(*offset as u64))
                    }
                }

                unreachable!()
            })
        }

        Ok(())
    }
}
