use bevy::prelude::*;
use bevy_node_editor::{
    NodeInput, NodeSet, NodeSlot, NodeTemplate,
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

        let var = if self.var.len() > 0 {
            &self.var
        } else {
            "0.0"
        };

        writeln!(&mut buf, "\treturn {};", self.output.transform(ShaderIO::Vec4, var, Some("1.0")))?;
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
    fn transform(self, target: ShaderIO, var: &str, extend: Option<&str>) -> String {
        let extend = extend.unwrap_or("0.0");

        match (self, target) {
            (Self::F32, Self::F32) => var.to_string(),
            (Self::F32, Self::Vec2) => format!("vec2<f32>({}, {})", var, extend),
            (Self::F32, Self::Vec3) => format!("vec3<f32>({}, vec2<f32>({}))", var, extend),
            (Self::F32, Self::Vec4) => format!("vec4<f32>({}, vec3<f32>({}))", var, extend),
            (_, Self::F32) => format!("{}.x", var),
            (Self::Vec2, Self::Vec2) => var.to_string(),
            (Self::Vec2, Self::Vec3) => format!("vec3<f32>({}, {})", var, extend),
            (Self::Vec2, Self::Vec4) => format!("vec4<f32>({}, vec2<f32>({}))", var, extend),
            (_, Self::Vec2) => format!("{}.xy", var),
            (Self::Vec3, Self::Vec3) => var.to_string(),
            (Self::Vec3, Self::Vec4) => format!("vec4<f32>({}, {})", var, extend),
            (_, Self::Vec3) => format!("{}.xyz", var),
            _ => var.to_string(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum ShaderNodes {
    Extend,
    MaterialPreview,
    #[default]
    Print,
    UV,
}

impl NodeSet for ShaderNodes {
    type NodeIO = ShaderBuilder;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match self {
            Self::Extend => {
                let mut builder = inputs["value"].clone();
                let input_var = builder.var;
                let input_io = builder.output;

                builder.output = input_io.extend();
                builder.var = format!("{}_{}", input_var, "extend");

                builder.content.push(format!("let {} = {};", builder.var, input_io.transform(builder.output, &input_var, Some("0.0"))));

                builder
            }
            Self::MaterialPreview => {
                inputs["input"].clone()
            },
            Self::Print => {
                let builder = inputs["output"].clone();
                let shader = builder.build().unwrap();

                println!("{}", shader);

                builder
            },
            Self::UV => {
                ShaderBuilder {
                    output: ShaderIO::Vec2,
                    var: "uv".to_string(),
                    ..default()
                }
            },
        }
    }
}

impl Into<NodeTemplate<ShaderNodes>> for ShaderNodes {
    fn into(self) -> NodeTemplate<ShaderNodes> {
        let preview_size = 400.0;
        let mut template = match self {
            Self::Extend => NodeTemplate {
                title: "Extend".to_string(),
                inputs: Some(vec![NodeInput::from_label("value")]),
                output_label: Some("vec".to_string()),
                ..default()
            },
            Self::MaterialPreview => NodeTemplate {
                title: "Preview".to_string(),
                inputs: Some(vec![NodeInput::from_label("input")]),
                width: preview_size,
                slot: Some(NodeSlot::new(preview_size)),
                ..default()
            },
            Self::Print => NodeTemplate {
                title: "Print".to_string(),
                inputs: Some(vec![NodeInput::from_label("output")]),
                ..default()
            },
            Self::UV => NodeTemplate {
                title: "UV".to_string(),
                output_label: Some("uv".to_string()),
                ..default()
            },
        };

        template.node = self;

        template
    }
}
