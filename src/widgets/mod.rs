use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_node_editor::{
    widget::SlotWidget,
    widgets::{InputWidget, InputWidgetPlugin, NumberInput},
};

use crate::shader::ShaderNodes;

mod material_preview_widget;

use material_preview_widget::MaterialPreviewWidgetPlugin;

#[derive(Default)]
pub struct WidgetPlugins;

impl PluginGroup for WidgetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MaterialPreviewWidgetPlugin)
            .add(InputWidgetPlugin::<ShaderNodes, NumberInput>::default())
    }
}

impl SlotWidget<Self, InputWidget<NumberInput>> for ShaderNodes {
    fn get_widget(&self) -> Option<InputWidget<NumberInput>> {
        match self {
            ShaderNodes::Extend(_) => Some(InputWidget::default()),
            _ => None,
        }
    }

    fn set_value(&mut self, value: NumberInput) {
        match self {
            Self::Extend(v) => *v = value,
            _ => {}
        }
    }
}
