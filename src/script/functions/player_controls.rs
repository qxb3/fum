use std::sync::Arc;

use crate::state::CurrentPlayerState;

use super::ScriptFnResult;

/// PLAY() control function.
pub fn play(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player.play().await.expect("Failed to call PLAY() function");
            }
        });

        Ok(())
    }
}

/// PLAY_PAUSE() control function.
pub fn play_pause(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player
                    .play_pause()
                    .await
                    .expect("Failed to call PLAY_PAUSE() function");
            }
        });

        Ok(())
    }
}

/// PAUSE() control function.
pub fn pause(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player
                    .pause()
                    .await
                    .expect("Failed to call PAUSE() function");
            }
        });

        Ok(())
    }
}

/// STOP() control function.
pub fn stop(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player.stop().await.expect("Failed to call STOP() function");
            }
        });

        Ok(())
    }
}

/// NEXT() control function.
pub fn next(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player.next().await.expect("Failed to call NEXT() function");
            }
        });

        Ok(())
    }
}

/// PREV() control function.
pub fn prev(current_player: CurrentPlayerState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let current_player = Arc::clone(&current_player);

        tokio::spawn(async move {
            let mut current_player = current_player.lock().await;
            if let Some(player) = current_player.as_mut() {
                player
                    .previous()
                    .await
                    .expect("Failed to call PREV() function");
            }
        });

        Ok(())
    }
}
