use crate::FumResult;

use super::{FumMode, FumModeEvent};

pub struct PlayerMode;

impl PlayerMode {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl FumMode for PlayerMode {
    async fn start(&mut self) -> FumResult<()> {
        Ok(())
    }

    async fn recv(&mut self) -> FumResult<FumModeEvent> {
        todo!()
    }
}
