use bevy::prelude::*;
use crate::physics::*;
use crate::arena::*;

// Ball movement
#[derive(Component)]
pub struct BallComponent;

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
        app.add_system_to_stage(CoreStage::PreUpdate, ball_movement);
    }
}


pub const SPEED_DAMP:f32 = 1.0;
fn ball_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut ball_query: Query<(Entity, &mut Transform, &mut BallComponent, &mut PhysicsObject)>,
    arena_query: Query<&Rotator>,
) {
    let arena = arena_query.single();

    ball_query.for_each_mut(|ball_query_res| {
        let (ball_entity, mut ball_transform, _, mut ball_component) = ball_query_res;

        let new_position = Vec2::new(ball_transform.translation.x, ball_transform.translation.z) + ball_component.speed * time.delta_seconds();
        
        ball_transform.translation.x = new_position.x;
        ball_transform.translation.z = new_position.y;
    
        ball_component.acc = ball_component.max_acc * Vec2::new(-1.0, 1.0) * Vec2::new(arena.angle.y, arena.angle.x) / ARENA_MAX_ANGLE;
    
        ball_component.speed = (ball_component.speed + ball_component.acc * time.delta_seconds()) * SPEED_DAMP;
    
        // check if out of bounds
        if ball_transform.translation.x < -5.0 || 
            ball_transform.translation.x < -5.0 ||
            ball_transform.translation.z < -5.0 ||
            ball_transform.translation.z < -5.0 {
                commands.entity(ball_entity).despawn();
            }
    });
    
}