use bevy::prelude::*;
use bevy_node_editor::{
    widgets::NumberInput,
    NodeInput,
    NodeOutput,
    NodeSet,
    NodeSlot,
    NodeTemplate,
};
use std::collections::HashMap;

use crate::shader::{ShaderBuilder, ShaderIO};

#[derive(Clone, Default, PartialEq)]
pub enum ShaderNodes {
    Component,
    Extend(NumberInput),
    MaterialPreview,
    Normal,
    #[default]
    Print,
    Saturate,
    Texture,
    UV,
}

impl NodeSet for ShaderNodes {
    type NodeIO = ShaderBuilder;

    fn resolve(
        &self,
        inputs: &HashMap<String, Self::NodeIO>,
        output: Option<&str>,
    ) -> Self::NodeIO {
        match self {
            Self::Component => {
                let output = output.unwrap();
                let mut builder = inputs["value"].clone();
                let input_var = builder.var;
                let input_io = builder.output;

                builder.output = ShaderIO::F32;
                builder.var = format!("{}_{}", input_var, output);

                builder.content.push(format!(
                    "let {} = {}.{};",
                    builder.var,
                    input_io.transform(ShaderIO::Vec4, &input_var, Some(0.0)),
                    output,
                ));

                builder
            }
            Self::Extend(input) => {
                let mut builder = inputs["value"].clone();
                let input_var = builder.var;
                let input_io = builder.output;

                builder.output = input_io.extend();
                builder.var = format!("{}_{}", input_var, "extend");

                builder.content.push(format!(
                    "let {} = {};",
                    builder.var,
                    input_io.transform(builder.output, &input_var, Some(input.value))
                ));

                builder
            }
            Self::MaterialPreview => inputs["input"].clone(),
            Self::Normal => ShaderBuilder {
                output: ShaderIO::Vec3,
                var: "world_normal".to_string(),
                ..default()
            },
            Self::Print => {
                let builder = inputs["output"].clone();
                let shader = builder.build().unwrap();

                println!("{}", shader);

                builder
            }
            Self::Saturate => {
                let mut builder = inputs["value"].clone();
                let input_var = builder.var;

                builder.var = format!("{}_{}", &input_var, "saturate");
                builder.content.push(format!(
                    "let {} = clamp({}, {}, {});",
                    builder.var,
                    input_var,
                    builder.output.fill(0.0),
                    builder.output.fill(1.0),
                ));

                builder
            }
            Self::Texture => {
                let mut var = "texture_color".to_string();
                let mut content = vec!["let texture_color = textureSample(texture, texture_sampler, uv);".to_string()];
                let output = output.unwrap();
                let mut io = ShaderIO::Vec4;

                if output != "color" {
                    var = format!("{}_{}", var, output);
                    content.push(format!("let {} = texture_color.{};", var, output));
                    io = ShaderIO::F32;
                }

                ShaderBuilder {
                    content,
                    output: io,
                    var,
                    ..default()
                }
            },
            Self::UV => ShaderBuilder {
                output: ShaderIO::Vec2,
                var: "uv".to_string(),
                ..default()
            },
        }
    }

    fn template(self) -> NodeTemplate<Self> {
        let preview_size = 400.0;
        let texture_size = 200.0;

        let mut template = match self {
            Self::Component => NodeTemplate {
                title: "Component".to_string(),
                inputs: Some(vec![NodeInput::from_label("value")]),
                outputs: Some(vec![
                    NodeOutput::from_label("x"),
                    NodeOutput::from_label("y"),
                    NodeOutput::from_label("z"),
                    NodeOutput::from_label("w"),
                ]),
                ..default()
            },
            Self::Extend(_) => NodeTemplate {
                title: "Extend".to_string(),
                inputs: Some(vec![NodeInput::from_label("value")]),
                outputs: Some(vec![NodeOutput::from_label("vec")]),
                slot: Some(NodeSlot::new(20.0)),
                ..default()
            },
            Self::MaterialPreview => NodeTemplate {
                title: "Preview".to_string(),
                inputs: Some(vec![NodeInput::from_label("input")]),
                width: preview_size,
                slot: Some(NodeSlot::new(preview_size)),
                ..default()
            },
            Self::Normal => NodeTemplate {
                title: "Normal".to_string(),
                outputs: Some(vec![NodeOutput::from_label("normal")]),
                ..default()
            },
            Self::Print => NodeTemplate {
                title: "Print".to_string(),
                inputs: Some(vec![NodeInput::from_label("output")]),
                ..default()
            },
            Self::Saturate => NodeTemplate {
                title: "Saturate".to_string(),
                inputs: Some(vec![NodeInput::from_label("value")]),
                outputs: Some(vec![NodeOutput::from_label("saturated")]),
                ..default()
            },
            Self::Texture => NodeTemplate {
                title: "Texture".to_string(),
                outputs: Some(vec![
                    NodeOutput::from_label("color"),
                    NodeOutput::from_label("r"),
                    NodeOutput::from_label("g"),
                    NodeOutput::from_label("b"),
                ]),
                slot: Some(NodeSlot::new(texture_size)),
                width: texture_size,
                ..default()
            },
            Self::UV => NodeTemplate {
                title: "UV".to_string(),
                outputs: Some(vec![NodeOutput::from_label("uv")]),
                ..default()
            },
        };

        template.node = self;

        template
    }
}
