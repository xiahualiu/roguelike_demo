use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowMode;

use roguelike_demo::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run()
}
