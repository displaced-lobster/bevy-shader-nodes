use bevy::prelude::*;
use bevy_node_editor::{widgets::NumberInput, NodeMenu};

use crate::shader::ShaderNodes;

#[derive(Default, Resource)]
pub struct Menu;

impl NodeMenu<ShaderNodes> for Menu {
    fn options(&self) -> Vec<(String, ShaderNodes)> {
        vec![
            (
                "Extend".to_string(),
                ShaderNodes::Extend(NumberInput::default()),
            ),
            ("Saturate".to_string(), ShaderNodes::Saturate),
            ("Component".to_string(), ShaderNodes::Component),
            ("Normal".to_string(), ShaderNodes::Normal),
            ("UV".to_string(), ShaderNodes::UV),
            ("Print".to_string(), ShaderNodes::Print),
            ("Preview".to_string(), ShaderNodes::MaterialPreview),
        ]
    }
}
