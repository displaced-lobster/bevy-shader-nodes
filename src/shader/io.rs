#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ShaderIO {
    #[default]
    F32,
    Vec2,
    Vec3,
    Vec4,
}

impl ShaderIO {
    pub fn extend(self) -> Self {
        match self {
            ShaderIO::F32 => ShaderIO::Vec2,
            ShaderIO::Vec2 => ShaderIO::Vec3,
            ShaderIO::Vec3 => ShaderIO::Vec4,
            ShaderIO::Vec4 => ShaderIO::Vec4,
        }
    }
    pub fn fill(&self, value: f32) -> String {
        match self {
            ShaderIO::F32 => format!("{}", value),
            ShaderIO::Vec2 => format!("vec2<f32>({:.5})", value),
            ShaderIO::Vec3 => format!("vec3<f32>({:.5})", value),
            ShaderIO::Vec4 => format!("vec4<f32>({:.5})", value),
        }
    }
    pub fn transform(self, target: ShaderIO, var: &str, extend: Option<f32>) -> String {
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
