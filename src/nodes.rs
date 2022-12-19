use bevy::prelude::*;
use bevy_node_editor::{
    widgets::NumberInput,
    NodeInput,
    NodeOutput,
    NodeSet,
    NodeSlot,
    NodeTemplate,
};
use color_eyre::eyre::Result;
use std::{collections::HashMap, io::Write};

#[derive(Clone, Default)]
pub struct ShaderBuilder {
    content: Vec<String>,
    output: ShaderIO,
    var: String,
}

impl ShaderBuilder {
    pub fn build(&self) -> Result<String> {
        let mut buf = Vec::new();

        writeln!(&mut buf, "@fragment")?;
        writeln!(&mut buf, "fn fragment(")?;
        writeln!(&mut buf, "\t#import bevy_pbr::mesh_vertex_output")?;
        writeln!(&mut buf, ") -> @location(0) vec4<f32> {{")?;

        for line in &self.content {
            writeln!(&mut buf, "\t{}", line)?;
        }

        let var = if self.var.len() > 0 { &self.var } else { "0.0" };

        writeln!(
            &mut buf,
            "\treturn {};",
            self.output.transform(ShaderIO::Vec4, var, Some(1.0))
        )?;
        writeln!(&mut buf, "}}")?;

        Ok(String::from_utf8(buf)?)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
enum ShaderIO {
    #[default]
    F32,
    Vec2,
    Vec3,
    Vec4,
}

impl ShaderIO {
    fn extend(self) -> Self {
        match self {
            ShaderIO::F32 => ShaderIO::Vec2,
            ShaderIO::Vec2 => ShaderIO::Vec3,
            ShaderIO::Vec3 => ShaderIO::Vec4,
            ShaderIO::Vec4 => ShaderIO::Vec4,
        }
    }
    fn fill(&self, value: f32) -> String {
        match self {
            ShaderIO::F32 => format!("{}", value),
            ShaderIO::Vec2 => format!("vec2<f32>({:.5})", value),
            ShaderIO::Vec3 => format!("vec3<f32>({:.5})", value),
            ShaderIO::Vec4 => format!("vec4<f32>({:.5})", value),
        }
    }
    fn transform(self, target: ShaderIO, var: &str, extend: Option<f32>) -> String {
        let extend = extend.unwrap_or(0.0);

        match (self, target) {
            (Self::F32, Self::F32) => var.to_string(),
            (Self::F32, Self::Vec2) => format!("vec2<f32>({}, {:.5})", var, extend),
            (Self::F32, Self::Vec3) => format!("vec3<f32>({}, vec2<f32>({:.5}))", var, extend),
            (Self::F32, Self::Vec4) => format!("vec4<f32>({}, vec3<f32>({:.5}))", var, extend),
            (_, Self::F32) => format!("{}.x", var),
            (Self::Vec2, Self::Vec2) => var.to_string(),
            (Self::Vec2, Self::Vec3) => format!("vec3<f32>({}, {:.5})", var, extend),
            (Self::Vec2, Self::Vec4) => format!("vec4<f32>({}, vec2<f32>({:.5}))", var, extend),
            (_, Self::Vec2) => format!("{}.xy", var),
            (Self::Vec3, Self::Vec3) => var.to_string(),
            (Self::Vec3, Self::Vec4) => format!("vec4<f32>({}, {:.5})", var, extend),
            (_, Self::Vec3) => format!("{}.xyz", var),
            _ => var.to_string(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum ShaderNodes {
    Component,
    Extend(NumberInput),
    MaterialPreview,
    Normal,
    #[default]
    Print,
    Saturate,
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
            Self::UV => ShaderBuilder {
                output: ShaderIO::Vec2,
                var: "uv".to_string(),
                ..default()
            },
        }
    }

    fn template(self) -> NodeTemplate<Self> {
        let preview_size = 400.0;
        let mut template = match self {
            Self::Component => NodeTemplate {
                title: "Vector".to_string(),
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
