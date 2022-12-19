use color_eyre::eyre::Result;
use std::io::Write;

use crate::shader::ShaderIO;

#[derive(Clone, Default)]
pub struct ShaderBuilder {
    pub content: Vec<String>,
    pub output: ShaderIO,
    pub var: String,
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
