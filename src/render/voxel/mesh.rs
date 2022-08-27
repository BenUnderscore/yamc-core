//Uses
use crate::world::chunk::size::*;
use crate::world::voxel::AttributeRegistry;
use crate::world::voxel::VoxelArray;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub struct SolidColorCubeModel {
    pub color: (f32, f32, f32),
}

impl SolidColorCubeModel {
    fn get_color_array(&self) -> [f32; 3] {
        [self.color.0, self.color.1, self.color.2]
    }
}

pub enum AppearanceAttribute {
    /// A regular solid block,
    SolidColorCube(SolidColorCubeModel),
    /// Completely transparent (air)
    None,
}

//The origin of this model is on the negative corner
const CUBE_VERTICES: [[f32; 3]; 36] = [
    //Bottom plane
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],

    //Top plane
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],

    [0.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0],

    //Front plane
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],

    [0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],

    //Back plane
    [0.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],

    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],

    //Left plane
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],

    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],

    //Right plane
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],

    [1.0, 0.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0]
];

fn append_solid_color_cube_model(
    vec: &mut Vec<Vertex>,
    solid_model: &SolidColorCubeModel,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
) {
    vec.reserve(CUBE_VERTICES.len());
    for vertex_position in CUBE_VERTICES {
        let translated_vertex_position = [vertex_position[0] + offset_x, vertex_position[1] + offset_y, vertex_position[2] + offset_z];
        vec.push(Vertex {
            position: translated_vertex_position,
            color: solid_model.get_color_array(),
        });
    }
}

pub(super) fn generate_mesh(
    array: &VoxelArray,
    appearance_registry: &AttributeRegistry<AppearanceAttribute>,
) -> Vec<Vertex> {
    let mut mesh = Vec::new();

    for x in 0..CHUNK_SIZE_X {
        for y in 0..CHUNK_SIZE_Y {
            for z in 0..CHUNK_SIZE_Z {
                let voxel = array.get_voxel_at_position(x, y, z);
                let voxel_appearance = appearance_registry.find(voxel.id).unwrap();
                match voxel_appearance {
                    AppearanceAttribute::SolidColorCube(solid_color_cube_model) => {
                        append_solid_color_cube_model(&mut mesh, solid_color_cube_model, x as f32, y as f32, z as f32)
                    }
                    AppearanceAttribute::None => (),
                }
            }
        }
    }

    mesh
}