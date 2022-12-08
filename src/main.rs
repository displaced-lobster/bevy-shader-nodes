use bevy::prelude::*;
use bevy_node_editor::{NodeMenuPlugin, NodePlugins, NodeSet, PanCameraPlugin};

mod menu;
mod nodes;
mod widgets;

use crate::{menu::Menu, nodes::ShaderNodes, widgets::WidgetPlugins};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<ShaderNodes>::default())
        .add_plugins(WidgetPlugins)
        .add_plugin(NodeMenuPlugin::<Menu, ShaderNodes>::default())
        .add_plugin(PanCameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(ShaderNodes::MaterialPreview.template());
}
