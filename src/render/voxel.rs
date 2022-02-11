use crate::world::voxel::VoxelAttributeRegistry;

pub struct VoxelSolidModel {
    color: (f32, f32, f32)
}

pub enum VoxelAppearanceAttribute {
    /// A regular solid block, 
    Solid(VoxelSolidModel),
    /// Completely transparent (air)
    None,
}

pub struct VoxelRenderer {
    appearance_registry: VoxelAttributeRegistry<VoxelAppearanceAttribute>,
}

impl VoxelRenderer {

    pub fn init(appearance_registry: VoxelAttributeRegistry<VoxelAppearanceAttribute>) -> VoxelRenderer {
        VoxelRenderer { appearance_registry }
    }
    
}