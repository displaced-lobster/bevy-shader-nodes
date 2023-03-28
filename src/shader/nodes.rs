use bevy::prelude::*;
use bevy_flow_node::{
    widgets::NumberInput,
    FlowNodeInput,
    FlowNodeOutput,
    FlowNodeSet,
    FlowNodeSlot,
    FlowNodeTemplate,
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
    Vector,
}

impl FlowNodeSet for ShaderNodes {
    type NodeIO = ShaderBuilder;

    fn resolve(
        &self,
        inputs: HashMap<String, Option<Self::NodeIO>>,
        output: Option<&str>,
    ) -> Self::NodeIO {
        let mut inputs = inputs;

        match self {
            Self::Component => {
                let output = output.unwrap();
                let mut builder = inputs
                    .remove("value")
                    .unwrap_or(None)
                    .unwrap_or(ShaderBuilder::default());
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
                let mut builder = inputs
                    .remove("value")
                    .unwrap_or(None)
                    .unwrap_or(ShaderBuilder::default());
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
            Self::MaterialPreview => inputs
                .remove("input")
                .unwrap_or(None)
                .unwrap_or(ShaderBuilder::default()),
            Self::Normal => {
                let mut content = Vec::new();
                let out = output.unwrap();

                let (io, var) = if out == "normal" {
                    (ShaderIO::Vec3, "world_normal".to_string())
                } else {
                    content.push(format!("let world_normal_{} = world_normal.{};", out, out));

                    (ShaderIO::F32, format!("world_normal_{}", out))
                };

                ShaderBuilder {
                    content,
                    output: io,
                    var,
                }
            }
            Self::Print => {
                let builder = inputs
                    .remove("output")
                    .unwrap_or(None)
                    .unwrap_or(ShaderBuilder::default());
                let shader = builder.build().unwrap();

                println!("{}", shader);

                builder
            }
            Self::Saturate => {
                let mut builder = inputs
                    .remove("value")
                    .unwrap_or(None)
                    .unwrap_or(ShaderBuilder::default());
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
                let mut content = vec![
                    "let texture_color = textureSample(texture, texture_sampler, uv);".to_string(),
                ];
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
                }
            }
            Self::UV => {
                let mut content = Vec::new();
                let out = output.unwrap();

                let (io, var) = if out == "uv" {
                    (ShaderIO::Vec2, "uv".to_string())
                } else {
                    content.push(format!("let uv_{} = uv.{};", out, out));

                    (ShaderIO::F32, format!("uv_{}", out))
                };

                ShaderBuilder {
                    content,
                    output: io,
                    var,
                }
            }
            Self::Vector => {
                static mut COUNTER: u32 = 0;

                let mut builder = ShaderBuilder {
                    output: ShaderIO::Vec4,
                    var: format!("vec_{}", unsafe { COUNTER }),
                    ..default()
                };

                unsafe {
                    COUNTER += 1;
                }

                let mut components = Vec::new();

                for input in ["x", "y", "z", "w"].iter() {
                    let mut value = inputs
                        .remove(*input)
                        .unwrap_or(None)
                        .unwrap_or(ShaderBuilder::default());

                    builder.content.append(&mut value.content);
                    components.push(
                        value
                            .output
                            .transform(ShaderIO::F32, &value.var, None)
                            .to_string(),
                    );
                }

                builder.content.push(format!(
                    "let {} = vec4({});",
                    builder.var,
                    components.join(", ")
                ));

                builder
            }
        }
    }

    fn template(self) -> FlowNodeTemplate<Self> {
        let preview_size = 400.0;
        let texture_size = 200.0;

        let mut template = match self {
            Self::Component => FlowNodeTemplate {
                title: "Component".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("value")]),
                outputs: Some(vec![
                    FlowNodeOutput::from_label("x"),
                    FlowNodeOutput::from_label("y"),
                    FlowNodeOutput::from_label("z"),
                    FlowNodeOutput::from_label("w"),
                ]),
                ..default()
            },
            Self::Extend(_) => FlowNodeTemplate {
                title: "Extend".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("value")]),
                outputs: Some(vec![FlowNodeOutput::from_label("vec")]),
                slot: Some(FlowNodeSlot::new(20.0)),
                ..default()
            },
            Self::MaterialPreview => FlowNodeTemplate {
                title: "Preview".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("input")]),
                width: preview_size,
                slot: Some(FlowNodeSlot::new(preview_size)),
                ..default()
            },
            Self::Normal => FlowNodeTemplate {
                title: "Normal".to_string(),
                outputs: Some(vec![
                    FlowNodeOutput::from_label("normal"),
                    FlowNodeOutput::from_label("x"),
                    FlowNodeOutput::from_label("y"),
                    FlowNodeOutput::from_label("z"),
                ]),
                ..default()
            },
            Self::Print => FlowNodeTemplate {
                title: "Print".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("output")]),
                ..default()
            },
            Self::Saturate => FlowNodeTemplate {
                title: "Saturate".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("value")]),
                outputs: Some(vec![FlowNodeOutput::from_label("saturated")]),
                ..default()
            },
            Self::Texture => FlowNodeTemplate {
                title: "Texture".to_string(),
                outputs: Some(vec![
                    FlowNodeOutput::from_label("color"),
                    FlowNodeOutput::from_label("r"),
                    FlowNodeOutput::from_label("g"),
                    FlowNodeOutput::from_label("b"),
                ]),
                slot: Some(FlowNodeSlot::new(texture_size)),
                width: texture_size,
                ..default()
            },
            Self::UV => FlowNodeTemplate {
                title: "UV".to_string(),
                outputs: Some(vec![
                    FlowNodeOutput::from_label("uv"),
                    FlowNodeOutput::from_label("x"),
                    FlowNodeOutput::from_label("y"),
                ]),
                ..default()
            },
            Self::Vector => FlowNodeTemplate {
                title: "Vector".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("x"),
                    FlowNodeInput::from_label("y"),
                    FlowNodeInput::from_label("z"),
                    FlowNodeInput::from_label("w"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("vec")]),
                ..default()
            },
        };

        template.node = self;

        template
    }
}
