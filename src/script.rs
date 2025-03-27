use std::path::PathBuf;

use rhai::{Engine, Scope, AST};

use crate::{widget::Widget, FumResult};

pub struct Script<'a> {
    /// Rhai engine.
    pub engine: Engine,

    /// Script scope.
    pub scope: Scope<'a>,

    /// Script ast.
    pub ast: AST,
}

impl<'a> Script<'a> {
    /// Creates a new script, loading from file.
    pub fn new<P: Into<PathBuf>>(config_path: P) -> FumResult<Self> {
        let engine = Engine::new();

        let mut scope = Scope::new();

        let ast = engine
            .compile_file(config_path.into())
            .map_err(|err| format!("Error parsing config script: {err}"))?;

        // Run the script and put all the stuff in the scope.
        engine
            .run_ast_with_scope(&mut scope, &ast)
            .map_err(|err| format!("Error executing config script: {err}"))?;

        Ok(Self {
            engine,
            scope,
            ast,
        })
    }

    /// Get & build ui widgets map.
    pub fn get_ui_widgets(&self) -> FumResult<Vec<Widget>> {
        let mut widgets: Vec<Widget> = Vec::new();

        // Get the `ui` variable from the script.
        let widget_arr = self
            .scope
            .get_value::<rhai::Array>("ui")
            .ok_or("ui needs to be an array of widget objects")?;

        for widget in widget_arr {
            Script::build_ui(widget, &mut widgets)?;
        }

        Ok(widgets)
    }

    /// A helper function for building the ui.
    fn build_ui(widget: rhai::Dynamic, widgets: &mut Vec<Widget>) -> FumResult<()> {
        // Turns the dynamic type into a map.
        let widget = widget
            .as_map_ref()
            .map_err(|_| "ui needs to be an array of widget objects")?;

        // Get the `type` from the widget.
        let widget_type = widget
            .get("type")
            .ok_or("widget needs a `type`")?
            .as_immutable_string_ref()
            .map_err(|_| "widget `type` needs to be a string")?;

        match widget_type.as_str() {
            // If the `type` is container.
            "container" => {
                // Get the container children.
                let children_arr = widget
                    .get("children")
                    .ok_or("container widget needs a `children`")?
                    .as_array_ref()
                    .map_err(|_| "container widget `children` needs to be an array of widget objects")?
                    .to_vec();

                let mut children: Vec<Widget> = Vec::new();

                for child in children_arr {
                    Script::build_ui(child, &mut children)?;
                }

                let container = Widget::Container {
                    children,
                };

                widgets.push(container);
            }

            // If the `type` is label.
            "label" => {
                let text = widget
                    .get("text")
                    .ok_or("label widget needs a `text`")?
                    .as_immutable_string_ref()
                    .map_err(|_| "label widget `text` needs to be a string")?
                    .to_string();

                let label = Widget::Label {
                    text,
                };

                widgets.push(label);
            }

            _ => {
                return Err(
                    format!("Unknown widget type: {}", widget_type.to_string()).into()
                )
            }
        }

        Ok(())
    }
}
