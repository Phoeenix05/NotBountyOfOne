use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PresentMode;

// #[derive(Bundle)]
// struct ProjectileBundle<M: Asset> {
//     projectile: Projectile<M>,
// }

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    transform: Transform,
}

fn spawn_projectile(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (mouse_x, mouse_y) = match window_query.get_single() {
        Ok(window) => window.cursor_position().unwrap().into(),
        Err(_) => return,
    };

    let transform = Transform::from_xyz(mouse_x, mouse_y, 0.0).with_scale(Vec3::splat(128.));
    dbg!(transform, (mouse_x, mouse_y));

    commands.spawn((
        Projectile {
            velocity: (1.0, 0.0).into(),
            transform,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform,
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
    ));
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Not... Bounty of One".into(),
            present_mode: PresentMode::Immediate,
            resolution: (1280.0, 800.0).into(),
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    });
    app.add_systems(Update, spawn_projectile);
    app.add_systems(Update, |mut query: Query<&Transform, With<Projectile>>| {
        println!("{:#?}", query)
    });

    app.run();
}
