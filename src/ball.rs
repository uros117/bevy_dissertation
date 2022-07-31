use bevy::prelude::*;
use crate::physics::*;
use crate::arena::*;

use super::GameState;

// Ball movement
#[derive(Component)]
pub struct BallComponent {
    pub start_pos: Transform,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball_comp: BallComponent,
    pub po: PhysicsObject,
    #[bundle]
    pub pbr: PbrBundle,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(ball_movement));
    }
}

pub fn reset_ball(
    game_state: &mut State<GameState>,
) {
    game_state.set(GameState::RespawnShrink).unwrap();
}

pub const SPEED_DAMP:f32 = 1.0;
fn ball_movement(
    mut _commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut ball_query: Query<(Entity, &mut Transform, &mut BallComponent, &mut PhysicsObject)>,
    arena_query: Query<&Rotator>,
) {
    let arena = arena_query.single();

    ball_query.for_each_mut(|ball_query_res| {
        let (_ball_entity, mut ball_transform, _, mut ball_po) = ball_query_res;

        let new_position = Vec2::new(ball_transform.translation.x, ball_transform.translation.z) + ball_po.speed * time.delta_seconds();
        
        ball_transform.translation.x = new_position.x;
        ball_transform.translation.z = new_position.y;
    
        ball_po.acc = ball_po.max_acc * Vec2::new(-1.0, 1.0) * Vec2::new(arena.angle.y, arena.angle.x) / ARENA_MAX_ANGLE;
    
        ball_po.speed = (ball_po.speed + ball_po.acc * time.delta_seconds()) * SPEED_DAMP;
    
        // check if out of bounds
        if ball_transform.translation.x < -ARENA_SIZE / 2.0 || 
            ball_transform.translation.x > ARENA_SIZE / 2.0 ||
            ball_transform.translation.z < -ARENA_SIZE / 2.0 ||
            ball_transform.translation.z > ARENA_SIZE / 2.0 {
                reset_ball(&mut game_state);
            }
    });
}