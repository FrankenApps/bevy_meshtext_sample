use bevy::prelude::*;
use meshtext::{MeshGenerator, MeshText, TextSection};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::Rgba {
            red: 1f32,
            green: 1f32,
            blue: 1f32,
            alpha: 1f32,
        }))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_system)
        .run();
}

#[derive(Component)]
struct RotationEntity;

/// set up a simple 3D scene with text
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let font_data = include_bytes!("../assets/font/FiraMono-Regular.ttf");
    let mut generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(1f32, 1f32, 0.2f32)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(&"Hello World!".to_string(), false, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();

    // text
    commands
        // use this bundle to change the rotation pivot to the center
        .spawn(PbrBundle {
            ..Default::default()
        })
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(1f32, 0f32, 0f32).into()),
                // transform mesh so that it is in the center
                transform: Transform::from_translation(Vec3::new(
                    text_mesh.bbox.size().x / -2f32,
                    0f32,
                    0f32,
                )),
                ..Default::default()
            });
        })
        .insert(RotationEntity);

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, With<RotationEntity>)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() as f32);
    }
}
