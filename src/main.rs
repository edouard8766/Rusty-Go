mod game;
mod board;

use bevy::prelude::*;
use board::Board;
use game::GoPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Go Game Bevy 0.18".into(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize  the global Background Color
        .insert_resource(ClearColor(Color::srgb(0.85, 0.7, 0.4))) 
        .init_resource::<Board>() // creates empty grid in memory
        .add_plugins(GoPlugin) // load game sustms
        .run();
}