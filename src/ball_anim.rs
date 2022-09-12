use bevy::prelude::*;
use bevy::utils::Instant;

use crate::ball::BallComponent;
use crate::physics::PhysicsObject;

use super::GameState;


pub struct BallAnimPlugin;

pub struct BallAnimatorAssets {
    start_time: Option<Instant>,
}

impl FromWorld for BallAnimatorAssets {
    fn from_world(_world: &mut World) -> Self {

        BallAnimatorAssets { start_time: None }
    }
}

impl Plugin for BallAnimPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BallAnimatorAssets>()
            .add_system_set(SystemSet::on_enter(GameState::RespawnGrow).with_system(ball_anim_grow_enter))
            .add_system_set(SystemSet::on_update(GameState::RespawnGrow).with_system(ball_anim_grow_update))
            .add_system_set(SystemSet::on_update(GameState::RespawnShrink).with_system(ball_anim_shrink_update));
    }
}

fn ball_anim_grow_enter(
    mut ball_query: Query<(&mut Transform, &mut BallComponent, &mut PhysicsObject)>
) {
    let (mut ball_transform, ball_component, mut ball_po) = ball_query.single_mut();
    ball_transform.translation = ball_component.start_pos.translation;
    ball_po.speed = Vec2::ZERO;
}

fn ball_anim_grow_update(
    mut game_state: ResMut<State<GameState>>,
    mut ball_anim_a: ResMut<BallAnimatorAssets>,
    mut ball_query: Query<(&mut Transform, With<BallComponent>)>
) {
    let (mut ball_transform, _) = ball_query.single_mut();

    if let None = ball_anim_a.start_time {
        ball_anim_a.start_time = Some(Instant::now());
    }

    if let Some(start_instant) = ball_anim_a.start_time {
        let delta = Instant::now() - start_instant;
        let scale = (delta.as_secs_f32()).clamp(0.0, 1.0);
        ball_transform.scale = scale * Vec3::ONE;
        if scale >= 1.0 {
            game_state.set(GameState::Running).unwrap();
            ball_anim_a.start_time = None;
        }
    }
}


fn ball_anim_shrink_update(
    mut game_state: ResMut<State<GameState>>,
    mut ball_anim_a: ResMut<BallAnimatorAssets>,
    mut ball_query: Query<(&mut Transform, With<BallComponent>)>
) {
    let (mut ball_transform, _) = ball_query.single_mut();

    if let None = ball_anim_a.start_time {
        ball_anim_a.start_time = Some(Instant::now());
    }

    if let Some(start_instant) = ball_anim_a.start_time {
        let delta = Instant::now() - start_instant;
        let scale = (1.0 - delta.as_secs_f32()).clamp(0.0, 1.0);
        ball_transform.scale = scale * Vec3::ONE;
        if scale <= 0.0 {
            game_state.set(GameState::RespawnGrow).unwrap();
            ball_anim_a.start_time = None;
        }
    }
}

