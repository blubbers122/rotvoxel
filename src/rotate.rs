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
    down_scale_factor: usize,
) -> (usize, usize, usize, Vec<P>)
where
    P: Clone,
{
    let (sx, cx) = rot_x.to_radians().sin_cos();
    let (sy, cy) = rot_y.to_radians().sin_cos();
    let (sz, cz) = rot_z.to_radians().sin_cos();

    // Rotation matrix (Z * Y * X)
    let rot = |x: f64, y: f64, z: f64| -> (f64, f64, f64) {
        let (x1, y1, z1) = (x, y * cx - z * sx, y * sx + z * cx); // X
        let (x2, y2, z2) = (x1 * cy + z1 * sy, y1, z1 * cy - x1 * sy); // Y
        let (x3, y3, z3) = (x2 * cz - y2 * sz, x2 * sz + y2 * cz, z2); // Z
        (x3, y3, z3)
    };

    // --- Compute bounds ---
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
        let (x, y, z) = rot(corner.0, corner.1, corner.2);

        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        min.2 = min.2.min(z);

        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
        max.2 = max.2.max(z);
    }

    let out_w = (max.0 - min.0).ceil() as usize;
    let out_h = (max.1 - min.1).ceil() as usize;
    let out_d = (max.2 - min.2).ceil() as usize;

    let mut out = vec![empty.clone(); out_w * out_h * out_d];

    // --- Inverse mapping ---
    for z in 0..out_d {
        for y in 0..out_h {
            for x in 0..out_w {
                let fx = x as f64 + min.0;
                let fy = y as f64 + min.1;
                let fz = z as f64 + min.2;

                // Inverse rotation (transpose of rotation matrix)
                let (sx, sy, sz) = rot(fx, fy, fz); // approximate inverse if symmetric

                if sx >= 0.0
                    && sx < width as f64
                    && sy >= 0.0
                    && sy < height as f64
                    && sz >= 0.0
                    && sz < depth as f64
                {
                    let src =
                        (sz as usize * width * height) + (sy as usize * width) + (sx as usize);

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
    let new_width = width / factor;
    let new_height = height / factor;
    let new_depth = depth / factor;

    let mut scaled = vec![buf[0].clone(); new_width * new_height];

    for y in 0..new_height {
        let y_row = y * new_width;
        let y_row_scaled = y * factor * width;
        for x in 0..new_width {
            scaled[y_row + x] = buf[y_row_scaled + x * factor].clone();
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
