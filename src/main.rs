#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, asset};


mod skybox;
mod obstacle;
mod physics;
mod arena;
mod ball;
mod hole;
mod ball_anim;
mod splash;
mod level;

use arena::*;
use ball::*;
use obstacle::*;
use hole::*;
use ball_anim::*;
use splash::*;
use level::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 800.0,
            title: String::from("Rolling Ball"),
            resizable: false,
            ..default()
        })
        .add_state(GameState::RespawnGrow)
        .add_plugins(DefaultPlugins)
        .add_plugin(skybox::SkyboxPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(HolePlugin)
        .add_plugin(BallAnimPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(LevelPlugin)
        .insert_resource(Msaa {samples: 4})
        .add_startup_system(asset_server_en_hotload)
        .run();
}

fn asset_server_en_hotload(
    asset_server: Res<AssetServer>
) {
    asset_server.watch_for_changes().unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    RespawnGrow,
    Running,
    RespawnShrink,
    Splash,
}
