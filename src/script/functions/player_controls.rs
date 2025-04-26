use std::sync::Arc;

use crate::state::FumState;

use super::ScriptFnResult;

/// PLAY() control function.
pub fn play(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
                player.play().await.expect("Failed to call PLAY() function");
            }
        });

        Ok(())
    }
}

/// PLAY_PAUSE() control function.
pub fn play_pause(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
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
pub fn pause(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
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
pub fn stop(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
                player.stop().await.expect("Failed to call STOP() function");
            }
        });

        Ok(())
    }
}

/// NEXT() control function.
pub fn next(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
                player.next().await.expect("Failed to call NEXT() function");
            }
        });

        Ok(())
    }
}

/// PREV() control function.
pub fn prev(state: FumState) -> impl Fn() -> ScriptFnResult<()> {
    move || -> ScriptFnResult<()> {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut state = state.lock().await;

            if let Some(player) = state.get_player_mut() {
                player
                    .previous()
                    .await
                    .expect("Failed to call PREV() function");
            }
        });

        Ok(())
    }
}
