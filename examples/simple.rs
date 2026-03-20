use std::collections::HashMap;

use dot_vox::{DotVoxData, Model, Voxel};
use image::{Rgba, RgbaImage};
use rotvoxel::{flatten_vox_model, flattened_voxels_colors_to_voxels, rotvoxel};

fn main() {
    // Open the image
    // let img = image::open("examples/threeforms.png").unwrap();
    let vox_data = dot_vox::load("examples/chr_knight.vox").unwrap();
    let model = &vox_data.models[0];
    let width = model.size.x as usize;
    let height = model.size.y as usize;
    let depth = model.size.z as usize;
    let pixels = flatten_vox_model(&vox_data)[0].clone();

    let unfound_color = [0u8; 4];
    let rotation_angle: f64 = 45.0; //Rotate in increments of 15 degrees
    let (rotated_width, rotated_height, rotated_depth, rotated) = rotvoxel(
        &pixels,
        &unfound_color, // The color for pixels that couldn't be found
        width,
        height,
        depth,
        rotation_angle,
        0.0,
        0.0,
    )
    .expect("Could not rotate sprite");

    let unflattened = flattened_voxels_colors_to_voxels(&rotated, rotated_width, rotated_height);

    // let rotated_image = RgbaImage::from_fn(rotated_width as u32, rotated_height as u32, |x, y| {
    //     Rgba(rotated[rotated_width * y as usize + x as usize])
    // });
    let mut color_to_pallete_index = HashMap::new();
    for (i, color) in vox_data.palette.iter().enumerate() {
        let color_array = [color.r, color.g, color.b, color.a];
        color_to_pallete_index.insert(color_array, i as u8);
    }

    let new_rotated_model = Model {
        voxels: unflattened
            .iter()
            .map(|v| Voxel {
                x: v.x as u8,
                y: v.y as u8,
                z: v.z as u8,
                i: *color_to_pallete_index.get(&v.color).unwrap(),
            })
            .collect(),
        size: dot_vox::Size {
            x: rotated_width as u32,
            y: rotated_height as u32,
            z: rotated_depth as u32,
        },
    };

    let new_rotated_vox = DotVoxData {
        models: vec![new_rotated_model],
        ..vox_data
    };

    let mut writer = std::fs::File::create("rotated.vox").unwrap();
    new_rotated_vox.write_vox(&mut writer).unwrap();
}
