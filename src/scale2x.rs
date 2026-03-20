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

/**
 * Get the indices of the neighbors of a voxel
 *
 * @param x The x coordinate of the voxel
 * @param y The y coordinate of the voxel
 * @param z The z coordinate of the voxel
 * @param width The width of the voxel buffer
 * @param height The height of the voxel buffer
 * @return A tuple of the indices of the neighbors in the order: center, up, left, down, right, forward, back
 */
fn get_neighbor_indices(
    x: usize,
    y: usize,
    z: usize,
    width: usize,
    height: usize,
) -> (usize, usize, usize, usize, usize, usize, usize) {
    let center = three_to_one(x, y, z, width, height);
    let up = three_to_one(x, y.saturating_sub(1), z, width, height);
    let left = three_to_one(x.saturating_sub(1), y, z, width, height);
    let down = three_to_one(x, y + 1, z, width, height);
    let right = three_to_one(x + 1, y, z, width, height);
    let forward = three_to_one(x, y, z + 1, width, height);
    let back = three_to_one(x, y, z.saturating_sub(1), width, height);
    (center, up, left, down, right, forward, back)
}

// Algorithm for fast upscaling of voxel art
pub fn scale2x<P>(
    buf: &[P],
    width: usize,
    height: usize,
    depth: usize,
) -> (usize, usize, usize, Vec<P>)
where
    P: Eq + Clone,
{
    let width2 = width * 2;
    let height2 = height * 2;
    let depth2 = depth * 2;

    let mut scaled = vec![buf[0].clone(); width2 * height2 * depth2];

    // Apply the algorithm to the center
    for z in 1..depth - 1 {
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let (center, up, left, down, right, forward, back) =
                    get_neighbor_indices(x, y, z, width, height);
                apply_scale2x_block(
                    &mut scaled,
                    three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                    width2,
                    height2,
                    (
                        // Center
                        &buf[center],
                        // Up
                        &buf[up],
                        // Left
                        &buf[left],
                        // Down
                        &buf[down],
                        // Right
                        &buf[right],
                        // Forward
                        &buf[forward],
                        // Back
                        &buf[back],
                    ),
                );
            }
        }
    }
    // it is TUBE time
    // in here, only z changes. so now we can just create a 'cornerless tube' growing along the z axis
    for z in 1..depth - 1 {
        for x in 1..width - 1 {
            // tube faces where y == 0 or y == height - 1
            let y = height - 1;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[up],
                    // Left
                    &buf[left],
                    // Down
                    &buf[center],
                    // Right
                    &buf[right],
                    // Forward
                    &buf[forward],
                    // Back
                    &buf[back],
                ),
            );

            let y = 0;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[center],
                    // Left
                    &buf[left],
                    // Down
                    &buf[down],
                    // Right
                    &buf[right],
                    // Forward
                    &buf[forward],
                    // Back
                    &buf[back],
                ),
            );
        }

        for y in 1..height - 1 {
            // tube faces where x == 0 or x == width - 1
            let x = width - 1;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[up],
                    // Left
                    &buf[left],
                    // Down
                    &buf[down],
                    // Right
                    &buf[center],
                    // Forward
                    &buf[forward],
                    // Back
                    &buf[back],
                ),
            );

            let x = 0;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[up],
                    // Left
                    &buf[center],
                    // Down
                    &buf[down],
                    // Right
                    &buf[right],
                    // Forward
                    &buf[forward],
                    // Back
                    &buf[back],
                ),
            );
        }
    }

    // the other 2 faces of the 'cube shell' started in the last nested loop
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            // tube faces where z == 0 or z == depth - 1
            let z = depth - 1;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[up],
                    // Left
                    &buf[left],
                    // Down
                    &buf[down],
                    // Right
                    &buf[right],
                    // Forward
                    &buf[center],
                    // Back
                    &buf[back],
                ),
            );

            let z = 0;
            let (center, up, left, down, right, forward, back) =
                get_neighbor_indices(x, y, z, width, height);
            apply_scale2x_block(
                &mut scaled,
                three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
                width2,
                height2,
                (
                    // Center
                    &buf[center],
                    // Up
                    &buf[up],
                    // Left
                    &buf[left],
                    // Down
                    &buf[down],
                    // Right
                    &buf[right],
                    // Forward
                    &buf[forward],
                    // Back
                    &buf[center],
                ),
            );
        }
    }

    // then, we apply the 12 corner strips

    // the 4 strips where only z changes
    for z in 1..depth - 1 {
        let x = width - 1;
        let y = height - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't got right or down
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[left],
                // Down
                &buf[center],
                // Right
                &buf[center],
                // Forward
                &buf[forward],
                // Back
                &buf[back],
            ),
        );

        let x = 0;
        let y = height - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go left or down
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[center],
                // Down
                &buf[center],
                // Right
                &buf[right],
                // Forward
                &buf[forward],
                // Back
                &buf[back],
            ),
        );

        let x = 0;
        let y = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go left or up
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[center],
                // Left
                &buf[center],
                // Down
                &buf[down],
                // Right
                &buf[right],
                // Forward
                &buf[forward],
                // Back
                &buf[back],
            ),
        );

        let x = width - 1;
        let y = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go right or up
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[center],
                // Left
                &buf[left],
                // Down
                &buf[down],
                // Right
                &buf[center],
                // Forward
                &buf[forward],
                // Back
                &buf[back],
            ),
        );
    }

    // the 4 strips where only y changes
    for y in 1..height - 1 {
        let x = width - 1;
        let z = depth - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go forward or right
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[left],
                // Down
                &buf[down],
                // Right
                &buf[center],
                // Forward
                &buf[center],
                // Back
                &buf[back],
            ),
        );

        let x = 0;
        let z = depth - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go left or forward
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[center],
                // Down
                &buf[down],
                // Right
                &buf[right],
                // Forward
                &buf[center],
                // Back
                &buf[back],
            ),
        );

        let x = 0;
        let z = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go left or back
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[center],
                // Down
                &buf[down],
                // Right
                &buf[right],
                // Forward
                &buf[forward],
                // Back
                &buf[center],
            ),
        );

        let x = width - 1;
        let z = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go right or back
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[left],
                // Down
                &buf[down],
                // Right
                &buf[center],
                // Forward
                &buf[forward],
                // Back
                &buf[center],
            ),
        );
    }

    // the 4 strips where only x changes
    for x in 1..width - 1 {
        let y = height - 1;
        let z = depth - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go forward or down
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[left],
                // Down
                &buf[center],
                // Right
                &buf[right],
                // Forward
                &buf[center],
                // Back
                &buf[back],
            ),
        );

        let y = 0;
        let z = depth - 1;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go up or forward
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[center],
                // Left
                &buf[left],
                // Down
                &buf[down],
                // Right
                &buf[right],
                // Forward
                &buf[center],
                // Back
                &buf[back],
            ),
        );

        let y = 0;
        let z = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go up or back
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[center],
                // Left
                &buf[left],
                // Down
                &buf[down],
                // Right
                &buf[right],
                // Forward
                &buf[forward],
                // Back
                &buf[center],
            ),
        );

        let y = height - 1;
        let z = 0;
        let (center, up, left, down, right, forward, back) =
            get_neighbor_indices(x, y, z, width, height);
        // we can't go down or back
        apply_scale2x_block(
            &mut scaled,
            three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
            width2,
            height2,
            (
                // Center
                &buf[center],
                // Up
                &buf[up],
                // Left
                &buf[left],
                // Down
                &buf[center],
                // Right
                &buf[right],
                // Forward
                &buf[forward],
                // Back
                &buf[center],
            ),
        );
    }

    // lastly, we apply the 8 corner voxels
    // for x in [0, width - 1] {
    //     for y in [0, height - 1] {
    //         for z in [0, depth - 1] {
    //             let (center, up, left, down, right, forward, back) =
    //                 get_neighbor_indices(x, y, z, width, height);
    //             apply_scale2x_block(
    //                 &mut scaled,
    //                 three_to_one(x, y, z, width2, height2), // scaled_y + x * 2,
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

// Convert a single voxel to an upscaled 2x2x2 block
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
    let c = center;

    (
        // (0,0,0)
        (if left == up && left != down && up != right {
            up
        } else {
            c
        })
        .clone(),
        // (1,0,0)
        (if up == right && up != left && right != down {
            right
        } else {
            c
        })
        .clone(),
        // (0,1,0)
        (if down == left && down != right && left != up {
            left
        } else {
            c
        })
        .clone(),
        // (1,1,0)
        (if right == down && right != up && down != left {
            down
        } else {
            c
        })
        .clone(),
        // (0,0,1) → +Z
        (if left == up && left == forward && left != down && up != right && forward != back {
            forward
        } else {
            c
        })
        .clone(),
        // (1,0,1)
        (if up == right && up == forward && up != left && right != down && forward != back {
            forward
        } else {
            c
        })
        .clone(),
        // (0,1,1)
        (if down == left && down == forward && down != right && left != up && forward != back {
            forward
        } else {
            c
        })
        .clone(),
        // (1,1,1)
        (if right == down && right == forward && right != up && down != left && forward != back {
            forward
        } else {
            c
        })
        .clone(),
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
