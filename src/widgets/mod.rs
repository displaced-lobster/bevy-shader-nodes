use bevy::app::{PluginGroup, PluginGroupBuilder};

mod material_preview_widget;

use material_preview_widget::MaterialPreviewWidgetPlugin;

#[derive(Default)]
pub struct WidgetPlugins;

impl PluginGroup for WidgetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(MaterialPreviewWidgetPlugin)
    }
}
