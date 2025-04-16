use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Container() widget function with opts.
pub fn container_opts() -> impl Fn(rhai::Map) -> ScriptFnResult<FumWidget> {
    move |opts: rhai::Map| -> ScriptFnResult<FumWidget> {
        // Extract container direction from opts.
        let direction = opts
            .get("direction")
            .cloned()
            .ok_or("Container widget needs to have a direction")?
            .try_cast_result::<taffy::FlexDirection>()
            .map_err(|_| "Container direction needs to be a valid direction")?;

        // Extract container alignment from opts, Will default to START if it doesnt exists.
        let align = opts
            .get("align")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(taffy::AlignItems::Start))
            .try_cast_result::<taffy::AlignItems>()
            .map_err(|_| "Container align needs to be a valid align value")?;

        // Extract container children from opts.
        let children = opts
            .get("children")
            .cloned()
            .ok_or("Container widget needs to have children")?
            .try_cast_result::<rhai::Array>()
            .map_err(|_| "Container children needs to be an array of widget")?;

        // Where the nodes of container children will be stored.
        let mut container_children = Vec::new();

        // width & height of container.
        let mut container_width = 0;
        let mut container_height = 0;

        for child in children {
            let child = child
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The children of the container needs to be a widget")?;

            // Adds the child size to the container size.
            container_width += child.get_width();
            container_height += child.get_height();

            container_children.push(child);
        }

        Ok(FumWidget::Container {
            children: container_children,
            direction,
            align,
            width: container_width,
            height: container_height,
        })
    }
}

/// Container() widget function.
pub fn container(
) -> impl Fn(taffy::FlexDirection, rhai::Array) -> ScriptFnResult<FumWidget> {
    move |direction: taffy::FlexDirection,
          children: rhai::Array|
          -> ScriptFnResult<FumWidget> {
        // Where the nodes of container children will be stored.
        let mut container_children = Vec::new();

        // width & height of container.
        let mut container_width = 0;
        let mut container_height = 0;

        for child in children {
            let child = child
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The children of the container needs to be a widget")?;

            // Adds the child size to the container size.
            container_width += child.get_width();
            container_height += child.get_height();

            container_children.push(child);
        }

        Ok(FumWidget::Container {
            children: container_children,
            direction,
            align: taffy::AlignItems::Start,
            width: container_width,
            height: container_height,
        })
    }
}
