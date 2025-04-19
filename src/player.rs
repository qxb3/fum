use std::time::Duration;

use crate::{
    status::{LoopStatus, PlaybackStatus},
    FumResult,
};

/// Trait that Player modes should implement.
#[async_trait::async_trait]
pub trait Player {
    async fn play(&mut self) -> FumResult<()>;
    async fn play_pause(&mut self) -> FumResult<()>;
    async fn pause(&mut self) -> FumResult<()>;
    async fn stop(&mut self) -> FumResult<()>;

    async fn next(&mut self) -> FumResult<()>;
    async fn previous(&mut self) -> FumResult<()>;

    async fn seek_forward(&mut self, offset: Duration) -> FumResult<()>;
    async fn seek_backward(&mut self, offset: Duration) -> FumResult<()>;
    async fn set_position(&mut self, trackid: &str, position: Duration) -> FumResult<()>;

    async fn playback_status(&self) -> FumResult<PlaybackStatus>;

    async fn loop_status(&self) -> FumResult<LoopStatus>;
    async fn set_loop_status(&mut self, loop_status: LoopStatus) -> FumResult<()>;

    async fn shuffle(&self) -> FumResult<bool>;
    async fn set_shuffle(&mut self, shuffle: bool) -> FumResult<()>;

    async fn volume(&self) -> FumResult<f64>;
    async fn set_volume(&mut self, volume: f64) -> FumResult<()>;

    async fn position(&self) -> FumResult<Duration>;
}
