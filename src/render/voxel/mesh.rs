//Uses
use crate::world::chunk::size::*;
use crate::world::voxel;
use crate::world::voxel::VoxelArray;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VoxelVertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub struct SolidModel {
    color: (f32, f32, f32),
}

pub enum AppearanceAttribute {
    /// A regular solid block,
    Solid(SolidModel),
    /// Completely transparent (air)
    None,
}

const CUBE_VERTICES: [[f32; 3]; 36] = [
    //Front quad (Z-)
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0],
    //Back quad (Z+)
    [1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
    //Top quad (Y+)
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 0.0],
    //Bottom quad (Y-)
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    //Left quad (X-)
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [0.0, 0.0, 1.0],
    //Right quad (X+)
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
];

pub fn generate_mesh(
    voxel_array: &VoxelArray,
    appearance_registry: &voxel::AttributeRegistry<AppearanceAttribute>,
) -> Vec<VoxelVertex> {
    let mut output = Vec::new();

    for x in 0..CHUNK_SIZE_X {
        for y in 0..CHUNK_SIZE_Y {
            for z in 0..CHUNK_SIZE_Z {
                let voxel = voxel_array.get_voxel_at_position(x, y, z);
                let appearance = appearance_registry.find(voxel.id).unwrap();

                match *appearance {
                    AppearanceAttribute::None => (),
                    AppearanceAttribute::Solid(model) => {
                        output.reserve(36);
                        let color_array = [model.color.0, model.color.1, model.color.2];

                        for position in CUBE_VERTICES.iter() {
                            output.push(VoxelVertex {
                                position: *position,
                                color: color_array,
                            });
                        }
                    }
                }
            }
        }
    }

    output
}