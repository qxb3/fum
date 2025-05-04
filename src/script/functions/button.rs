use anyhow::anyhow;

use crate::{
    event::EventSender,
    widget::{FumWidgetKind, SendSyncFnPtr},
};

/// raw function of BUTTON().
pub fn button_function_raw(
    event_sender: EventSender,
) -> impl Fn(rhai::Map) -> Option<FumWidgetKind> {
    move |opts: rhai::Map| -> Option<FumWidgetKind> {
        // Extract widget from opts.
        let widget_opt = match opts.get("widget").cloned() {
            Some(widget_opt) => widget_opt,
            None => {
                event_sender
                    .send(Err(anyhow!("Button widget needs to have a `widget`")))
                    .unwrap();

                return None;
            }
        };

        // Parse widget.
        let widget_kind = match widget_opt.try_cast_result::<Option<FumWidgetKind>>() {
            Ok(widget) => match widget {
                Some(widget) => widget,
                None => return None,
            },
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Button `widget` needs to be a valid widget")))
                    .unwrap();

                return None;
            }
        };

        // Extract func from opts.
        let func_opt = match opts.get("func").cloned() {
            Some(func_opt) => func_opt,
            None => {
                event_sender
                    .send(Err(anyhow!("Button widget needs to have a `func`")))
                    .unwrap();

                return None;
            }
        };

        // Parse func.
        let func = match func_opt.try_cast_result::<rhai::FnPtr>() {
            Ok(func) => SendSyncFnPtr::new(func),
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Button `func` needs to be a valid function")))
                    .unwrap();

                return None;
            }
        };

        Some(FumWidgetKind::Button {
            widget_kind: Box::new(widget_kind),
            func,
        })
    }
}

/// BUTTON() a wrapper around raw.
pub fn button_function(
    event_sender: EventSender,
) -> impl Fn(Option<FumWidgetKind>, rhai::FnPtr) -> Option<FumWidgetKind> {
    move |widget: Option<FumWidgetKind>, func: rhai::FnPtr| -> Option<FumWidgetKind> {
        let mut opts = rhai::Map::new();
        opts.insert("widget".into(), rhai::Dynamic::from(widget));
        opts.insert("func".into(), rhai::Dynamic::from(func));

        let raw = button_function_raw(event_sender.clone());
        raw(opts)
    }
}
