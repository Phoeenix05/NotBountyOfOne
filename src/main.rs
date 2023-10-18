use bevy::prelude::*;
use bevy::window::PresentMode;

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

    app.run();
}
