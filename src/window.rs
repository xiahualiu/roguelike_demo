use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_setup);
    }
}

fn window_setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set windows size
    let mut window = windows.single_mut();
    window.title = String::from("Roguelike demo");

    // Set camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
            ..default()
        },
        ..default()
    });
}
