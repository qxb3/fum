use unicode_width::UnicodeWidthStr;

use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Button() widget function.
pub fn button() -> impl Fn(rhai::Dynamic, rhai::FnPtr) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic, func: rhai::FnPtr| -> ScriptFnResult<FumWidget> {
        let text = text.to_string();

        Ok(FumWidget::Button {
            text: text.to_string(),
            func,
            width: UnicodeWidthStr::width(text.as_str()) as u16,
            height: 1,
        })
    }
}
