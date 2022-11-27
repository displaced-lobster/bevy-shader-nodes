use bevy::prelude::*;
use bevy_node_editor::{
    NodeMenu, NodeTemplate
};

use crate::{nodes::ShaderNodes, widgets:: MaterialPreviewWidget};

#[derive(Default, Resource)]
pub struct Menu;

impl NodeMenu<ShaderNodes> for Menu {
    fn build(&self, commands: &mut Commands, node: &ShaderNodes) {
        let template: NodeTemplate<ShaderNodes> = (*node).clone().into();
        let entity = commands.spawn(template).id();

        if *node == ShaderNodes::MaterialPreview {
            commands.entity(entity).insert(MaterialPreviewWidget::default());
        }
    }

    fn options(&self) -> Vec<(String, ShaderNodes)> {
        vec![
            ("Extend".to_string(), ShaderNodes::Extend),
            ("UV".to_string(), ShaderNodes::UV),
            ("Print".to_string(), ShaderNodes::Print),
            ("Preview".to_string(), ShaderNodes::MaterialPreview),
        ]
    }
}
