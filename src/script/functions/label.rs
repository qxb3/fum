use unicode_width::UnicodeWidthStr;

use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Label() widget function.
pub fn label() -> impl Fn(rhai::Dynamic) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic| -> ScriptFnResult<FumWidget> {
        let text = text.to_string();

        Ok(FumWidget::Label {
            text: text.to_string(),
            width: UnicodeWidthStr::width(text.as_str()) as u16,
            height: 1,
        })
    }
}
