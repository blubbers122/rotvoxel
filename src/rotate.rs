use std::f64;

pub enum Axis {
    X,
    Y,
    Z,
}

// Algorithm for rotating the image
#[multiversion::multiversion(targets("x86_64+sse3", "x86_64+sse3+avx", "x86_64+sse3+avx2"))]
pub fn rotate<P>(
    voxels: &[P],
    empty: &P,
    width: usize,
    height: usize,
    depth: usize,
    rot_x: f64,
    rot_y: f64,
    rot_z: f64,
    _down_scale_factor: usize,
) -> (usize, usize, usize, Vec<P>)
where
    P: Clone,
{
    let (sin_x, cos_x) = rot_x.to_radians().sin_cos();
    let (sin_y, cos_y) = rot_y.to_radians().sin_cos();
    let (sin_z, cos_z) = rot_z.to_radians().sin_cos();

    // Center of the source model
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;
    let cz = depth as f64 / 2.0;

    // Forward rotation matrix R = Rz * Ry * Rx (applied around model center)
    let rot_fwd = |x: f64, y: f64, z: f64| -> (f64, f64, f64) {
        // Rx
        let (x1, y1, z1) = (x, y * cos_x - z * sin_x, y * sin_x + z * cos_x);
        // Ry
        let (x2, y2, z2) = (x1 * cos_y + z1 * sin_y, y1, z1 * cos_y - x1 * sin_y);
        // Rz
        let (x3, y3, z3) = (x2 * cos_z - y2 * sin_z, x2 * sin_z + y2 * cos_z, z2);
        (x3, y3, z3)
    };

    // Inverse rotation matrix R^-1 = Rx^T * Ry^T * Rz^T (transpose = negate sines, reverse order)
    let rot_inv = |x: f64, y: f64, z: f64| -> (f64, f64, f64) {
        // Rz^T
        let (x1, y1, z1) = (x * cos_z + y * sin_z, -x * sin_z + y * cos_z, z);
        // Ry^T
        let (x2, y2, z2) = (x1 * cos_y - z1 * sin_y, y1, x1 * sin_y + z1 * cos_y);
        // Rx^T
        let (x3, y3, z3) = (x2, y2 * cos_x + z2 * sin_x, -y2 * sin_x + z2 * cos_x);
        (x3, y3, z3)
    };

    // --- Compute output bounds by forward-rotating all 8 corners (centered) ---
    let mut min = (f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

    for &corner in &[
        (0.0, 0.0, 0.0),
        (width as f64, 0.0, 0.0),
        (0.0, height as f64, 0.0),
        (0.0, 0.0, depth as f64),
        (width as f64, height as f64, 0.0),
        (width as f64, 0.0, depth as f64),
        (0.0, height as f64, depth as f64),
        (width as f64, height as f64, depth as f64),
    ] {
        // Rotate around model center
        let (rx, ry, rz) = rot_fwd(corner.0 - cx, corner.1 - cy, corner.2 - cz);

        min.0 = min.0.min(rx);
        min.1 = min.1.min(ry);
        min.2 = min.2.min(rz);

        max.0 = max.0.max(rx);
        max.1 = max.1.max(ry);
        max.2 = max.2.max(rz);
    }

    let out_w = (max.0 - min.0).ceil() as usize;
    let out_h = (max.1 - min.1).ceil() as usize;
    let out_d = (max.2 - min.2).ceil() as usize;

    let mut out = vec![empty.clone(); out_w * out_h * out_d];

    // --- Inverse mapping: for each output voxel, find the source voxel ---
    for z in 0..out_d {
        for y in 0..out_h {
            for x in 0..out_w {
                // Output coordinate in rotated space (relative to rotated center)
                let fx = x as f64 + min.0;
                let fy = y as f64 + min.1;
                let fz = z as f64 + min.2;

                // Inverse rotate to find source coordinates, then add back center offset
                let (src_x, src_y, src_z) = rot_inv(fx, fy, fz);
                let src_x = src_x + cx;
                let src_y = src_y + cy;
                let src_z = src_z + cz;

                if src_x >= 0.0
                    && src_x < width as f64
                    && src_y >= 0.0
                    && src_y < height as f64
                    && src_z >= 0.0
                    && src_z < depth as f64
                {
                    let src =
                        (src_z as usize * width * height) + (src_y as usize * width) + (src_x as usize);

                    let dst = (z * out_w * out_h) + (y * out_w) + x;

                    out[dst] = voxels[src].clone();
                }
            }
        }
    }

    (out_w, out_h, out_d, out)
}

pub fn downscale<P>(
    buf: &[P],
    width: usize,
    height: usize,
    depth: usize,
    factor: usize,
) -> (usize, usize, usize, Vec<P>)
where
    P: Clone,
{
    let new_width = (width / factor).max(1);
    let new_height = (height / factor).max(1);
    let new_depth = (depth / factor).max(1);

    let mut scaled = vec![buf[0].clone(); new_width * new_height * new_depth];

    for z in 0..new_depth {
        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x * factor;
                let src_y = y * factor;
                let src_z = z * factor;
                let src_idx = src_z * width * height + src_y * width + src_x;
                let dst_idx = z * new_width * new_height + y * new_width + x;
                scaled[dst_idx] = buf[src_idx].clone();
            }
        }
    }

    (new_width, new_height, new_depth, scaled)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn rotation_90_deg() {
//         let (w, h, new) = rotate90(&[1, 2, 3, 4, 5, 6], 3, 2);
//         assert_eq!(w, 2);
//         assert_eq!(h, 3);
//         assert_eq!(new, [4, 1, 5, 2, 6, 3]);
//     }

//     #[test]
//     fn rotation_180_deg() {
//         let (w, h, new) = rotate180(&[1, 2, 3, 4, 5, 6], 3, 2);
//         assert_eq!(w, 3);
//         assert_eq!(h, 2);
//         assert_eq!(new, [6, 5, 4, 3, 2, 1]);
//     }

//     #[test]
//     fn rotation_270_deg() {
//         let (w, h, new) = rotate270(&[1, 2, 3, 4, 5, 6], 3, 2);
//         assert_eq!(w, 2);
//         assert_eq!(h, 3);
//         assert_eq!(new, [3, 6, 2, 5, 1, 4]);
//     }
// }
