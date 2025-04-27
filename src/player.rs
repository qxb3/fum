use std::{any::Any, time::Duration};

use crate::{
    mpris::TrackId,
    status::{LoopStatus, PlaybackStatus},
    FumResult,
};

/// Trait that Player modes should implement.
#[async_trait::async_trait]
pub trait Player: Any + Send + Sync {
    async fn play(&mut self) -> FumResult<()>;
    async fn play_pause(&mut self) -> FumResult<()>;
    async fn pause(&mut self) -> FumResult<()>;
    async fn stop(&mut self) -> FumResult<()>;

    async fn next(&mut self) -> FumResult<()>;
    async fn previous(&mut self) -> FumResult<()>;

    async fn seek_forward(&mut self, offset: Duration) -> FumResult<()>;
    async fn seek_backward(&mut self, offset: Duration) -> FumResult<()>;
    async fn set_position(&mut self, track_id: &TrackId, position: Duration) -> FumResult<()>;

    async fn playback_status(&self) -> FumResult<PlaybackStatus>;

    async fn loop_status(&self) -> FumResult<LoopStatus>;
    async fn set_loop_status(&mut self, loop_status: LoopStatus) -> FumResult<()>;

    async fn shuffle(&self) -> FumResult<bool>;
    async fn set_shuffle(&mut self, shuffle: bool) -> FumResult<()>;

    async fn volume(&self) -> FumResult<f64>;
    async fn set_volume(&mut self, volume: f64) -> FumResult<()>;

    async fn position(&self) -> FumResult<Duration>;

    // WORKAROUND: #[async_trait] can interfere with standard downcasting via Any.
    /// Returns a reference to the `Any` trait object.
    ///
    /// This method serves as a workaround when using the `#[async_trait]` macro,
    /// which can prevent direct coercion from `&dyn Player` to `&dyn Any`.
    /// Calling this method explicitly provides the `&dyn Any` reference needed
    /// to use the standard downcasting methods like `Any::downcast_ref`.
    ///
    /// You typically call this *before* attempting to downcast:
    /// `player_ref.as_any().downcast_ref::<FooPlayer>()`.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
