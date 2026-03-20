//! Voxel Art rotation algorithms that works with many types of voxel buffers.
//!
//! This library allows you to rotate voxel art using the [rotsprite](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite) algorithm.

#[doc(hidden)]
pub mod flatten_vox;
#[doc(hidden)]
pub mod rotate;
#[doc(hidden)]
pub mod scale2x;

pub use crate::flatten_vox::*;
pub use crate::rotate::*;
pub use crate::scale2x::*;
use std::collections::HashMap;
use thiserror::Error;

use dot_vox::{DotVoxData, Model, Voxel};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("image size doesn't match with supplied width")]
    ImageSizeMismatch,
}

/// Default number of Scale2x passes (3 passes = 8x upscale).
const DEFAULT_SCALE_PASSES: u32 = 3;

/// Rotate a voxel buffer using the rotsprite algorithm.
///
/// Upscales the buffer using Scale2x, rotates at the higher resolution,
/// then downscales back. This produces smoother edges than naive rotation.
///
/// - `buf`: flat voxel buffer in (x, y, z) order
/// - `empty_color`: the "empty" voxel value (e.g. transparent)
/// - `width`, `height`, `depth`: dimensions of the voxel buffer
/// - `rot_x`, `rot_y`, `rot_z`: rotation angles in degrees
/// - `scale_passes`: number of Scale2x passes (each pass doubles resolution).
///   Use `None` for the default (3 passes = 8x). 2 passes = 4x is faster with slightly less smoothing.
#[multiversion::multiversion(
    targets("x86_64+sse3", "x86_64+sse3+avx", "x86_64+sse3+avx2"),
    dispatcher = "static"
)]
pub fn rotvoxel<P>(
    buf: &[P],
    empty_color: &P,
    width: usize,
    height: usize,
    depth: usize,
    rot_x: f64,
    rot_y: f64,
    rot_z: f64,
    scale_passes: Option<u32>,
) -> Result<(usize, usize, usize, Vec<P>), Error>
where
    P: Eq + Clone,
{
    // If there's no rotation we don't have to do anything
    if rot_x == 0.0 && rot_y == 0.0 && rot_z == 0.0 {
        return Ok((width, height, depth, buf.to_vec()));
    }

    let len = buf.len();
    if len % width != 0 {
        return Err(Error::ImageSizeMismatch);
    }

    let passes = scale_passes.unwrap_or(DEFAULT_SCALE_PASSES);
    let down_scale_factor = 1 << passes; // 2^passes

    // Upscale using the scale2x algorithm
    let (mut sw, mut sh, mut sd, mut scaled) = (width, height, depth, buf.to_vec());
    for _ in 0..passes {
        let result = scale2x(&scaled, sw, sh, sd, empty_color);
        sw = result.0;
        sh = result.1;
        sd = result.2;
        scaled = result.3;
    }

    // Rotate the upscaled model
    let (rotated_width, rotated_height, rotated_depth, rotated) = rotate(
        &scaled,
        empty_color,
        sw,
        sh,
        sd,
        rot_x,
        rot_y,
        rot_z,
        down_scale_factor,
    );

    // Downscale back to approximately original resolution
    let (out_width, out_height, out_depth, out) = downscale(
        &rotated,
        rotated_width,
        rotated_height,
        rotated_depth,
        down_scale_factor,
    );

    Ok((out_width, out_height, out_depth, out))
}

/// Rotate a DotVox model and return a new DotVoxData.
///
/// This is a convenience function that handles the flatten/unflatten/palette
/// boilerplate for working with .vox files directly.
///
/// - `vox_data`: the loaded DotVoxData
/// - `model_index`: which model to rotate (usually 0)
/// - `rot_x`, `rot_y`, `rot_z`: rotation angles in degrees
/// - `scale_passes`: number of Scale2x passes (None = default 3 = 8x upscale)
pub fn rotvoxel_dotvox(
    vox_data: &DotVoxData,
    model_index: usize,
    rot_x: f64,
    rot_y: f64,
    rot_z: f64,
    scale_passes: Option<u32>,
) -> Result<DotVoxData, Error> {
    let model = &vox_data.models[model_index];
    let width = model.size.x as usize;
    let height = model.size.y as usize;
    let depth = model.size.z as usize;
    let pixels = flatten_vox_model(vox_data)[model_index].clone();

    let empty_color = [0u8; 4];
    let (rotated_width, rotated_height, rotated_depth, rotated) = rotvoxel(
        &pixels,
        &empty_color,
        width,
        height,
        depth,
        rot_x,
        rot_y,
        rot_z,
        scale_passes,
    )?;

    let unflattened = flattened_voxels_colors_to_voxels(&rotated, rotated_width, rotated_height);

    // Build a reverse palette lookup
    let mut color_to_palette_index = HashMap::new();
    for (i, color) in vox_data.palette.iter().enumerate() {
        let color_array = [color.r, color.g, color.b, color.a];
        color_to_palette_index.insert(color_array, i as u8);
    }

    let new_model = Model {
        voxels: unflattened
            .iter()
            .filter_map(|v| {
                color_to_palette_index.get(&v.color).map(|&i| Voxel {
                    x: v.x as u8,
                    y: v.y as u8,
                    z: v.z as u8,
                    i,
                })
            })
            .collect(),
        size: dot_vox::Size {
            x: rotated_width as u32,
            y: rotated_height as u32,
            z: rotated_depth as u32,
        },
    };

    Ok(DotVoxData {
        version: vox_data.version,
        index_map: vox_data.index_map.clone(),
        models: vec![new_model],
        palette: vox_data.palette.clone(),
        materials: vox_data.materials.clone(),
        scenes: vox_data.scenes.clone(),
        layers: vox_data.layers.clone(),
    })
}
