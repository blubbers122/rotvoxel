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
    // it is TUBE time
    // in here, only z changes. so now we can just create a 'cornerless tube' growing along the z axis
    // for z in 1..depth - 1 {
    //     for x in 1..width - 1 {
    //         // tube faces where y == 0 or y == height - 1
    //         let y = height - 1;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[up],
    //                 // Left
    //                 &buf[left],
    //                 // Down
    //                 &buf[center],
    //                 // Right
    //                 &buf[right],
    //                 // Forward
    //                 &buf[forward],
    //                 // Back
    //                 &buf[back],
    //             ),
    //         );

    //         let y = 0;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[center],
    //                 // Left
    //                 &buf[left],
    //                 // Down
    //                 &buf[down],
    //                 // Right
    //                 &buf[right],
    //                 // Forward
    //                 &buf[forward],
    //                 // Back
    //                 &buf[back],
    //             ),
    //         );
    //     }

    //     for y in 1..height - 1 {
    //         // tube faces where x == 0 or x == width - 1
    //         let x = width - 1;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[up],
    //                 // Left
    //                 &buf[left],
    //                 // Down
    //                 &buf[down],
    //                 // Right
    //                 &buf[center],
    //                 // Forward
    //                 &buf[forward],
    //                 // Back
    //                 &buf[back],
    //             ),
    //         );

    //         let x = 0;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[up],
    //                 // Left
    //                 &buf[center],
    //                 // Down
    //                 &buf[down],
    //                 // Right
    //                 &buf[right],
    //                 // Forward
    //                 &buf[forward],
    //                 // Back
    //                 &buf[back],
    //             ),
    //         );
    //     }
    // }

    // // the other 2 faces of the 'cube shell' started in the last nested loop
    // for x in 1..width - 1 {
    //     for y in 1..height - 1 {
    //         // tube faces where z == 0 or z == depth - 1
    //         let z = depth - 1;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[up],
    //                 // Left
    //                 &buf[left],
    //                 // Down
    //                 &buf[down],
    //                 // Right
    //                 &buf[right],
    //                 // Forward
    //                 &buf[center],
    //                 // Back
    //                 &buf[back],
    //             ),
    //         );

    //         let z = 0;
    //         let (center, up, left, down, right, forward, back) =
    //             get_neighbor_indices(x, y, z, width, height);
    //         apply_scale2x_block(
    //             &mut scaled,
    //             three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //             width2,
    //             height2,
    //             (
    //                 // Center
    //                 &buf[center],
    //                 // Up
    //                 &buf[up],
    //                 // Left
    //                 &buf[left],
    //                 // Down
    //                 &buf[down],
    //                 // Right
    //                 &buf[right],
    //                 // Forward
    //                 &buf[forward],
    //                 // Back
    //                 &buf[center],
    //             ),
    //         );
    //     }
    // }

    // // then, we apply the 12 corner strips

    // // the 4 strips where only z changes
    // for z in 1..depth - 1 {
    //     let x = width - 1;
    //     let y = height - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't got right or down
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[center],
    //             // Right
    //             &buf[center],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let x = 0;
    //     let y = height - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go left or down
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[center],
    //             // Down
    //             &buf[center],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let x = 0;
    //     let y = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go left or up
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[center],
    //             // Left
    //             &buf[center],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let x = width - 1;
    //     let y = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go right or up
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[center],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[center],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[back],
    //         ),
    //     );
    // }

    // // the 4 strips where only y changes
    // for y in 1..height - 1 {
    //     let x = width - 1;
    //     let z = depth - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go forward or right
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[center],
    //             // Forward
    //             &buf[center],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let x = 0;
    //     let z = depth - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go left or forward
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[center],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[center],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let x = 0;
    //     let z = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go left or back
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[center],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[center],
    //         ),
    //     );

    //     let x = width - 1;
    //     let z = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go right or back
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[center],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[center],
    //         ),
    //     );
    // }

    // // the 4 strips where only x changes
    // for x in 1..width - 1 {
    //     let y = height - 1;
    //     let z = depth - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go forward or down
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[center],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[center],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let y = 0;
    //     let z = depth - 1;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go up or forward
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[center],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[center],
    //             // Back
    //             &buf[back],
    //         ),
    //     );

    //     let y = 0;
    //     let z = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go up or back
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[center],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[down],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[center],
    //         ),
    //     );

    //     let y = height - 1;
    //     let z = 0;
    //     let (center, up, left, down, right, forward, back) =
    //         get_neighbor_indices(x, y, z, width, height);
    //     // we can't go down or back
    //     apply_scale2x_block(
    //         &mut scaled,
    //         three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //         width2,
    //         height2,
    //         (
    //             // Center
    //             &buf[center],
    //             // Up
    //             &buf[up],
    //             // Left
    //             &buf[left],
    //             // Down
    //             &buf[center],
    //             // Right
    //             &buf[right],
    //             // Forward
    //             &buf[forward],
    //             // Back
    //             &buf[center],
    //         ),
    //     );
    // }

    // lastly, we apply the 8 corner voxels
    // for x in [0, width - 1] {
    //     for y in [0, height - 1] {
    //         for z in [0, depth - 1] {
    //             let (center, up, left, down, right, forward, back) =
    //                 get_neighbor_indices(x, y, z, width, height);
    //             apply_scale2x_block(
    //                 &mut scaled,
    //                 three_to_one(x * 2, y * 2, z * 2, width2, height2), // scaled_y + x * 2,
    //                 width2,
    //                 height2,
    //                 (
    //                     // Center
    //                     &buf[center],
    //                     // Up
    //                     &buf[up],
    //                     // Left
    //                     &buf[left],
    //                     // Down
    //                     &buf[down],
    //                     // Right
    //                     &buf[right],
    //                     // Forward
    //                     &buf[forward],
    //                     // Back
    //                     &buf[back],
    //                 ),
    //             );
    //         }
    //     }
    // }

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
    // center
    scaled[pos] = block_pixels.0;
    // right
    scaled[pos + 1] = block_pixels.1;
    // down
    scaled[pos + width] = block_pixels.2;
    // down right
    scaled[pos + width + 1] = block_pixels.3;
    // forward
    scaled[pos + width * height] = block_pixels.4;
    // forward right
    scaled[pos + width * height + 1] = block_pixels.5;
    // forward down
    scaled[pos + width * height + width] = block_pixels.6;
    // forward down right
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
    // For each corner, nx/ny/nz are the toward-neighbors, ox/oy/oz are away-neighbors.
    // Check XY, XZ, YZ planes independently using the Scale2x rule.
    #[inline(always)]
    fn corner<P: Eq + Clone>(nx: &P, ny: &P, nz: &P, ox: &P, oy: &P, oz: &P, c: &P) -> P {
        // XY plane: nx == ny && ny != oy && nx != ox
        if nx == ny && ny != oy && nx != ox { return nx.clone(); }
        // XZ plane: nx == nz && nz != oz && nx != ox
        if nx == nz && nz != oz && nx != ox { return nx.clone(); }
        // YZ plane: ny == nz && nz != oz && ny != oy
        if ny == nz && nz != oz && ny != oy { return ny.clone(); }
        c.clone()
    }

    let c = center;

    (
        // (0,0,0): toward left,up,back — away right,down,forward
        corner(left, up, back, right, down, forward, c),
        // (1,0,0): toward right,up,back — away left,down,forward
        corner(right, up, back, left, down, forward, c),
        // (0,1,0): toward left,down,back — away right,up,forward
        corner(left, down, back, right, up, forward, c),
        // (1,1,0): toward right,down,back — away left,up,forward
        corner(right, down, back, left, up, forward, c),
        // (0,0,1): toward left,up,forward — away right,down,back
        corner(left, up, forward, right, down, back, c),
        // (1,0,1): toward right,up,forward — away left,down,back
        corner(right, up, forward, left, down, back, c),
        // (0,1,1): toward left,down,forward — away right,up,back
        corner(left, down, forward, right, up, back, c),
        // (1,1,1): toward right,down,forward — away left,up,back
        corner(right, down, forward, left, up, back, c),
    )
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn scale2x_test() {
//         let buf = [1, 2, 3, 4];
//         let (w, h, new) = scale2x(&buf, 2, 2);
//         assert_eq!(w, 4);
//         assert_eq!(h, 4);
//         assert_eq!(new, [1, 1, 2, 2, 1, 1, 2, 2, 3, 3, 4, 4, 3, 3, 4, 4]);

//         let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//         let (_, _, new) = scale2x(&buf, 3, 3);
//         let mut cmp = Vec::<usize>::new();
//         cmp.extend([1, 1, 2, 2, 3, 3].iter());
//         cmp.extend([1, 1, 2, 2, 3, 3].iter());
//         cmp.extend([4, 4, 5, 5, 6, 6].iter());
//         cmp.extend([4, 4, 5, 5, 6, 6].iter());
//         cmp.extend([7, 7, 8, 8, 9, 9].iter());
//         cmp.extend([7, 7, 8, 8, 9, 9].iter());
//         assert_eq!(new, cmp);

//         let buf = [1, 2, 3, 4, 5, 6];
//         let (_, _, new) = scale2x(&buf, 3, 2);
//         assert_eq!(
//             new,
//             [1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 4, 4, 5, 5, 6, 6]
//         );
//     }
// }
