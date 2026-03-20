pub fn three_to_one(x: usize, y: usize, z: usize, width: usize, height: usize) -> usize {
    x + y * width + z * width * height
}

pub fn one_to_three(index: usize, width: usize, height: usize) -> (usize, usize, usize) {
    let z = index / (width * height);
    let index = index % (width * height);
    let y = index / width;
    let x = index % width;
    (x, y, z)
}

// Algorithm for fast upscaling of voxel art
pub fn scale2x<P>(
    buf: &[P],
    width: usize,
    height: usize,
    depth: usize,
    empty: &P,
) -> (usize, usize, usize, Vec<P>)
where
    P: Eq + Clone,
{
    let width2 = width * 2;
    let height2 = height * 2;
    let depth2 = depth * 2;

    let mut scaled = vec![empty.clone(); width2 * height2 * depth2];

    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                let center = &buf[three_to_one(x, y, z, width, height)];
                let up = if y > 0 { &buf[three_to_one(x, y - 1, z, width, height)] } else { empty };
                let down = if y < height - 1 { &buf[three_to_one(x, y + 1, z, width, height)] } else { empty };
                let left = if x > 0 { &buf[three_to_one(x - 1, y, z, width, height)] } else { empty };
                let right = if x < width - 1 { &buf[three_to_one(x + 1, y, z, width, height)] } else { empty };
                let forward = if z < depth - 1 { &buf[three_to_one(x, y, z + 1, width, height)] } else { empty };
                let back = if z > 0 { &buf[three_to_one(x, y, z - 1, width, height)] } else { empty };

                apply_scale2x_block(
                    &mut scaled,
                    three_to_one(x * 2, y * 2, z * 2, width2, height2),
                    width2,
                    height2,
                    (center, up, left, down, right, forward, back),
                );
            }
        }
    }

    (width2, height2, depth2, scaled)
}

// Apply the block on the buffer
#[inline(always)]
fn apply_scale2x_block<P>(
    scaled: &mut [P],
    pos: usize,
    width: usize,
    height: usize,
    pixels: (&P, &P, &P, &P, &P, &P, &P),
) where
    P: Eq + Clone,
{
    let block_pixels = calculate_scale2x_block(
        pixels.0, pixels.1, pixels.2, pixels.3, pixels.4, pixels.5, pixels.6,
    );
    scaled[pos] = block_pixels.0;
    scaled[pos + 1] = block_pixels.1;
    scaled[pos + width] = block_pixels.2;
    scaled[pos + width + 1] = block_pixels.3;
    scaled[pos + width * height] = block_pixels.4;
    scaled[pos + width * height + 1] = block_pixels.5;
    scaled[pos + width * height + width] = block_pixels.6;
    scaled[pos + width * height + width + 1] = block_pixels.7;
}

// Convert a single voxel to an upscaled 2x2x2 block using 3D Scale2x.
//
// Each output corner has 3 "toward" neighbors and 3 "away" neighbors.
// We check the standard Scale2x rule independently on each of the 3
// axis-planes (XY, XZ, YZ). If any plane triggers, we smooth that corner.
//
// 2D Scale2x rule for a pair of toward-neighbors (n1, n2) with
// away-neighbors (o1, o2):
//   n1 == n2 && n2 != o2 && n1 != o1  →  use n1
//
// Since when multiple planes trigger they always agree on the value
// (the shared toward-neighbor forces equality), we just pick the first match.
#[inline(always)]
fn calculate_scale2x_block<P>(
    center: &P,
    up: &P,
    left: &P,
    down: &P,
    right: &P,
    forward: &P,
    back: &P,
) -> (P, P, P, P, P, P, P, P)
where
    P: Eq + Clone,
{
    #[inline(always)]
    fn corner<P: Eq + Clone>(nx: &P, ny: &P, nz: &P, ox: &P, oy: &P, oz: &P, c: &P) -> P {
        if nx == ny && ny != oy && nx != ox { return nx.clone(); }
        if nx == nz && nz != oz && nx != ox { return nx.clone(); }
        if ny == nz && nz != oz && ny != oy { return ny.clone(); }
        c.clone()
    }

    let c = center;

    (
        corner(left, up, back, right, down, forward, c),
        corner(right, up, back, left, down, forward, c),
        corner(left, down, back, right, up, forward, c),
        corner(right, down, back, left, up, forward, c),
        corner(left, up, forward, right, down, back, c),
        corner(right, up, forward, left, down, back, c),
        corner(left, down, forward, right, up, back, c),
        corner(right, down, forward, left, up, back, c),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale2x_uniform_1x1x1() {
        let buf = vec![1];
        let (w, h, d, result) = scale2x(&buf, 1, 1, 1, &0);
        assert_eq!((w, h, d), (2, 2, 2));
        assert_eq!(result, vec![1; 8]);
    }

    #[test]
    fn scale2x_uniform_2x2x2() {
        let buf = vec![5; 8];
        let (w, h, d, result) = scale2x(&buf, 2, 2, 2, &0);
        assert_eq!((w, h, d), (4, 4, 4));
        assert!(result.iter().all(|&v| v == 5));
    }

    #[test]
    fn scale2x_preserves_dimensions() {
        let buf = vec![0; 3 * 2 * 2];
        let (w, h, d, result) = scale2x(&buf, 3, 2, 2, &0);
        assert_eq!((w, h, d), (6, 4, 4));
        assert_eq!(result.len(), 6 * 4 * 4);
    }

    #[test]
    fn scale2x_single_voxel_in_empty() {
        let mut buf = vec![0; 27]; // 3x3x3
        buf[three_to_one(1, 1, 1, 3, 3)] = 1;
        let (w, h, d, result) = scale2x(&buf, 3, 3, 3, &0);
        assert_eq!((w, h, d), (6, 6, 6));
        for dz in 0..2 {
            for dy in 0..2 {
                for dx in 0..2 {
                    let idx = three_to_one(2 + dx, 2 + dy, 2 + dz, 6, 6);
                    assert_eq!(result[idx], 1, "at ({}, {}, {})", 2 + dx, 2 + dy, 2 + dz);
                }
            }
        }
    }

    #[test]
    fn scale2x_edge_smoothing() {
        let mut buf = vec![0; 27]; // 3x3x3
        buf[three_to_one(1, 1, 1, 3, 3)] = 1;
        buf[three_to_one(2, 1, 1, 3, 3)] = 1;
        let (_w, _h, _d, result) = scale2x(&buf, 3, 3, 3, &0);
        let filled: usize = result.iter().filter(|&&v| v == 1).count();
        assert!(filled >= 16, "expected >= 16 filled, got {}", filled);
    }
}
