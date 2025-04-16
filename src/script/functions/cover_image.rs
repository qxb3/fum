use crate::widget::FumWidget;

use super::ScriptFnResult;

/// CoverImage() widget function.
pub fn cover_image() -> impl Fn(rhai::INT, rhai::INT) -> ScriptFnResult<FumWidget> {
    move |width: rhai::INT, height: rhai::INT| -> ScriptFnResult<FumWidget> {
        Ok(FumWidget::CoverImage {
            width: width as u16,
            height: height as u16,
        })
    }
}
