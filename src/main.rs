mod post_processing_hdr;

use bevy::prelude::*;
use post_processing_hdr::{setup_scene, update_bloom_settings, bounce_spheres};

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(update_bloom_settings)
        .add_system(bounce_spheres)
        .run();
}
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .run();
//         // App::new().add_system(hello_world_system).run();
// }
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // plane
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(shape::Plane::from_size(5.0).into()),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });
//     // cube
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
//         material: materials.add(Color::rgb(0.8, 0.0, 0.0).into()),
//         transform: Transform::from_xyz(0.0, 1.5, 0.0),
//         ..default()
//     });
//     // light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 5_000.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
//     // camera
//     commands.spawn(Camera3dBundle {
//         camera: Camera {
//             hdr: true, // 1. HDR is required for bloom
//             ..default()
//         },
//         transform: Transform::from_xyz(-3.0, 3.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }
// // fn hello_world_system() {
// //     println!("hello world");
// // }