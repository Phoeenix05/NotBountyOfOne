use bevy::{
    prelude::*,
    reflect::TypePath,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RayHit, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

#[derive(Resource, Default)]
struct WorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

#[derive(Reflect)]
struct NotBooRaycastSet;

fn world_coords(
    mut world_coords: ResMut<WorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_coords.0 = world_position;
    }
}

fn spawn_projectile(
    buttons: Res<Input<MouseButton>>,
    world_coords: Res<WorldCoords>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 0.0)
                .with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(RaycastMesh::<NotBooRaycastSet>::default());
}

fn despawn_projectile<T: TypePath + Send + Sync>(
    buttons: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Handle<Mesh>, &mut Handle<ColorMaterial>), With<RaycastMesh<T>>>,
) {
    // for (mesh_handle, mut material_handle) in &mut query.iter() {
    //     if let Some(_) = meshes.get(mesh_handle) {
    //         let new_material = ColorMaterial::from(Color::WHITE);
    //         *material_handle = materials.add(new_material);
    //     }
    // }
}

fn update_raycast(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<NotBooRaycastSet>>,
) {
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}

fn main() {
    let mut app = App::new();

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Not... Bounty of One".into(),
            present_mode: bevy::window::PresentMode::Immediate,
            resolution: (1280.0, 800.0).into(),
            ..default()
        }),
        ..default()
    };
    app.add_plugins((
        DefaultPlugins.set(window_plugin),
        DefaultRaycastingPlugin::<NotBooRaycastSet>::default(),
    ));

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_systems(Startup, |mut commands: Commands| {
        commands.init_resource::<WorldCoords>();
        commands
            .spawn((Camera2dBundle::default(), MainCamera))
            .insert(RaycastSource::<NotBooRaycastSet>::new());
    });
    app.add_systems(
        First,
        update_raycast.before(RaycastSystem::BuildRays::<NotBooRaycastSet>),
    );
    app.add_systems(Update, world_coords);
    app.add_systems(Update, spawn_projectile);
    app.add_systems(Update, despawn_projectile::<NotBooRaycastSet>);
    // app.add_systems(Update, print_intersections::<NotBooRaycastSet>);

    app.run();
}
