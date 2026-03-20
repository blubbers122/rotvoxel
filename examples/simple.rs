use rotvoxel::rotvoxel_dotvox;

fn main() {
    let vox_data = dot_vox::load("examples/chr_knight.vox").unwrap();
    let rotation_angle: f64 = 45.0;

    let rotated_vox = rotvoxel_dotvox(
        &vox_data,
        0,               // model index
        0.0,             // rot_x
        0.0,             // rot_y
        rotation_angle,  // rot_z
        None,            // scale_passes (default = 3 = 8x)
    )
    .expect("Could not rotate voxel model");

    let mut writer = std::fs::File::create("rotated.vox").unwrap();
    rotated_vox.write_vox(&mut writer).unwrap();
}
