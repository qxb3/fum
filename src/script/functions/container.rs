use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Container() widget function with opts.
pub fn container_opts() -> impl Fn(rhai::Map) -> ScriptFnResult<FumWidget> {
    move |opts: rhai::Map| -> ScriptFnResult<FumWidget> {
        // Extract container direction from opts.
        let direction = opts
            .get("direction")
            .cloned()
            .ok_or("Container widget needs to have a `direction`")?
            .try_cast_result::<taffy::FlexDirection>()
            .map_err(|_| "Container `direction` needs to be a valid value")?;

        // Extract container alignment from opts, Will default to START if it doesnt exists.
        let align = opts
            .get("align")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(taffy::AlignItems::Start))
            .try_cast_result::<taffy::AlignItems>()
            .map_err(|_| "Container `align` needs to be a valid align value")?;

        // Extract the container spacing from opts, Will default to 0 if it doesnt exists.
        let spacing = opts
            .get("spacing")
            .cloned()
            .unwrap_or(rhai::Dynamic::from_int(0))
            .as_int()
            .map_err(|_| "Container `spacing` needs to be a valid number")?;

        // Extract the container justify from opts.
        let justify = opts
            .get("justify")
            .cloned()
            .and_then(|j| j.try_cast::<taffy::JustifyContent>());

        // Extract container children from opts.
        let children = opts
            .get("children")
            .cloned()
            .ok_or("Container widget needs to have `children`")?
            .try_cast_result::<rhai::Array>()
            .map_err(|_| "Container `children` needs to be an array of widget")?;

        // Where the nodes of container children will be stored.
        let mut container_children = Vec::new();
        for child in children {
            let child = child
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The `children` of the container needs to be a widget")?;

            container_children.push(child);
        }

        Ok(FumWidget::Container {
            children: container_children,
            direction,
            align,
            justify,
            spacing: spacing as u16,
        })
    }
}

/// Container() widget function with default values.
pub fn container(
) -> impl Fn(taffy::FlexDirection, rhai::Array) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("children".into(), rhai::Dynamic::from(children));

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}

/// Container() widget function with default values & can pass more options.
pub fn container_ext_opts(
) -> impl Fn(taffy::FlexDirection, rhai::Array, rhai::Map) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array,
          ext_opts: rhai::Map|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("children".into(), rhai::Dynamic::from(children));
        opts.extend(ext_opts);

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}

/// ContainerCenter() widget function. A normal container but with align set to CENTER.
pub fn container_center(
) -> impl Fn(taffy::FlexDirection, rhai::Array) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("align".into(), rhai::Dynamic::from(taffy::AlignItems::Center));
        opts.insert("children".into(), rhai::Dynamic::from(children));

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}

/// ContainerCenter() widget function. A normal container but with align set to CENTER & can pass more opts.
pub fn container_center_ext_opts(
) -> impl Fn(taffy::FlexDirection, rhai::Array, rhai::Map) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array,
          ext_opts: rhai::Map|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("align".into(), rhai::Dynamic::from(taffy::AlignItems::Center));
        opts.insert("children".into(), rhai::Dynamic::from(children));
        opts.extend(ext_opts);

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}

/// ContainerEnd() widget function. A normal container but with align set to END.
pub fn container_end(
) -> impl Fn(taffy::FlexDirection, rhai::Array) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("align".into(), rhai::Dynamic::from(taffy::AlignItems::End));
        opts.insert("children".into(), rhai::Dynamic::from(children));

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}

/// ContainerEnd() widget function. A normal container but with align set to END & can pass more opts.
pub fn container_end_ext_opts(
) -> impl Fn(taffy::FlexDirection, rhai::Array, rhai::Map) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array,
          ext_opts: rhai::Map|
          -> ScriptFnResult<FumWidget> {
        // Build up the opts.
        let mut opts = rhai::Map::new();
        opts.insert("direction".into(), rhai::Dynamic::from(direction));
        opts.insert("align".into(), rhai::Dynamic::from(taffy::AlignItems::End));
        opts.insert("children".into(), rhai::Dynamic::from(children));
        opts.extend(ext_opts);

        // Call the container_opts function.
        let container_opts = container_opts();
        container_opts(opts)
    }
}
