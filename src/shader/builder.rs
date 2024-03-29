use color_eyre::eyre::Result;
use std::io::Write;

use crate::shader::ShaderIO;

#[derive(Clone)]
pub struct ShaderBuilder {
    pub content: Vec<String>,
    pub output: ShaderIO,
    pub var: String,
}

impl Default for ShaderBuilder {
    fn default() -> Self {
        Self {
            content: vec![],
            output: ShaderIO::Vec4,
            var: "ZERO".to_string(),
        }
    }
}

const SHADER_PRELUDE: &str = r#"
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

const ZERO = vec4<f32>(0.0, 0.0, 0.0, 0.0);

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
"#;

impl ShaderBuilder {
    pub fn build(&self) -> Result<String> {
        let mut buf = Vec::new();

        write!(buf, "{}", SHADER_PRELUDE)?;

        for line in &self.content {
            writeln!(&mut buf, "    {}", line)?;
        }

        let var = if !self.var.is_empty() {
            &self.var
        } else {
            "0.0"
        };

        writeln!(
            &mut buf,
            "    return vec4<f32>({}, 1.0);",
            self.output.transform(ShaderIO::Vec3, var, Some(0.0))
        )?;
        writeln!(&mut buf, "}}")?;

        Ok(String::from_utf8(buf)?)
    }
}
