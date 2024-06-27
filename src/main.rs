// Disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use roguelike_demo::GamePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
