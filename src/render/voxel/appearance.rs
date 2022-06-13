pub struct SolidModel {
    pub color: (f32, f32, f32),
}

pub enum AppearanceAttribute {
    /// A regular solid block,
    Solid(SolidModel),
    /// Completely transparent (air)
    None,
}