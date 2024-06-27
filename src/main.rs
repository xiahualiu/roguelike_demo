// Disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowMode;

use roguelike_demo::GamePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
