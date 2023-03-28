use bevy::prelude::*;
use bevy_flow_node::{widgets::NumberInput, FlowNodeMenu};

use crate::shader::ShaderNodes;

#[derive(Default, Resource)]
pub struct Menu;

impl FlowNodeMenu<ShaderNodes> for Menu {
    fn options(&self) -> Vec<(String, ShaderNodes)> {
        vec![
            ("Normal".to_string(), ShaderNodes::Normal),
            ("UV".to_string(), ShaderNodes::UV),
            ("Texture".to_string(), ShaderNodes::Texture),
            (
                "Extend".to_string(),
                ShaderNodes::Extend(NumberInput::default()),
            ),
            ("Saturate".to_string(), ShaderNodes::Saturate),
            ("Component".to_string(), ShaderNodes::Component),
            ("Vector".to_string(), ShaderNodes::Vector),
            ("Print".to_string(), ShaderNodes::Print),
            ("Preview".to_string(), ShaderNodes::MaterialPreview),
        ]
    }
}
