use dot_vox::DotVoxData;

use crate::{one_to_three, scale2x::three_to_one};

pub fn flatten_vox_model(vox_data: &DotVoxData) -> Vec<Vec<[u8; 4]>> {
    let mut result = Vec::new();

    for model in &vox_data.models {
        // start out completely transparent
        let mut model_vox_colors_flattened =
            vec![[0, 0, 0, 0]; (model.size.x * model.size.y * model.size.z) as usize];

        for voxel in &model.voxels {
            let index = three_to_one(
                voxel.x as usize,
                voxel.y as usize,
                voxel.z as usize,
                model.size.x as usize,
                model.size.y as usize,
            );
            let color = vox_data.palette[voxel.i as usize];
            model_vox_colors_flattened[index] = [color.r, color.g, color.b, color.a];
        }
        result.push(model_vox_colors_flattened);
    }
    result
}

pub struct MyVoxel {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub color: [u8; 4],
}

pub fn flattened_voxels_colors_to_voxels(
    flattened_voxels: &Vec<[u8; 4]>,
    width: usize,
    height: usize,
) -> Vec<MyVoxel> {
    let mut result = Vec::<MyVoxel>::new();

    for (i, voxel_color) in flattened_voxels.iter().enumerate() {
        // skip transparent voxels
        if voxel_color[3] == 0 {
            continue;
        }
        let (x, y, z) = one_to_three(i, width, height);
        result.push(MyVoxel {
            x,
            y,
            z,
            color: *voxel_color,
        });
    }
    result
}
