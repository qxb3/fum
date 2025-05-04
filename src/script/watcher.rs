use std::{path::PathBuf, sync::mpsc};

use anyhow::anyhow;
use notify::{
    event::CreateKind, recommended_watcher, EventKind, INotifyWatcher, RecursiveMode, Watcher,
};

use crate::{
    event::{Event, EventSender, ScriptEvent},
    FumResult,
};

/// A wrapper around notify crate.
pub struct ConfigWatcher {
    /// A notify watcher event sender. This is just here to be `alive`
    /// And won't be using directly.
    #[allow(dead_code)]
    sender: mpsc::Sender<notify::Result<notify::Event>>,

    /// Config watcher.
    watcher: INotifyWatcher,
}

impl ConfigWatcher {
    pub fn new(event_sender: EventSender) -> FumResult<Self> {
        let (sender, _) = mpsc::channel();

        let watcher = recommended_watcher(move |res: notify::Result<notify::Event>| match res {
            Ok(event) => match event.kind {
                // Only send out the config modify event on file creations and modification of the config path.
                EventKind::Create(CreateKind::File) | EventKind::Modify(_) => {
                    event_sender
                        .send(Ok(Event::Script(ScriptEvent::ConfigModified)))
                        .unwrap();
                }
                _ => {}
            },
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Error on config watcher event: {err}")))
                    .unwrap();
            }
        })?;

        Ok(Self { sender, watcher })
    }

    pub fn watch(&mut self, config_path: &PathBuf) -> FumResult<()> {
        self.watcher
            .watch(config_path.as_path(), RecursiveMode::NonRecursive)?;

        Ok(())
    }
}
