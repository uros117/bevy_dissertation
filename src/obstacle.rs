use bevy::prelude::*;
use crate::physics::*;
use crate::ball::*;


#[derive(Component)]
pub struct ObstacleComponent;

#[derive(Bundle)]
pub struct ObstacleBundle {
    pub obstacle_comp: ObstacleComponent,
    pub po: PhysicsObject,
    #[bundle]
    pub pbr: PbrBundle,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::PostUpdate, obstacle_system);
    }
}

fn obstacle_system(
    mut ball_query: Query<(&mut Transform, &mut PhysicsObject, &BallComponent)>,
    mut obstacle_query: Query<(&mut Transform, &mut PhysicsObject, &ObstacleComponent, Without<BallComponent>)>,
) {
    let mut ball_iterator = ball_query.iter_mut();
    // Grab all ball entitys
    while let Some(mut ball) = ball_iterator.next()  {
        let mut other_iter = obstacle_query.iter_mut();
        // Grab other coliders
        while let Some(mut other) = other_iter.next()  {
            resolve_colission(ball.0.as_mut(), ball.1.as_mut(), other.0.as_mut(), other.1.as_mut());
        }
    }
}

fn resolve_colission(tr_a: &mut Transform, po_a: &mut PhysicsObject, tr_b: &mut Transform, po_b: &mut PhysicsObject) {
    if let Colider::CircleColider(ball_r) = po_a.colider {
        match po_b.colider {
            Colider::BoxColider(box_w, box_h) => {
                // BALL vs BOX 
                let diff = Vec2::new(
                    (-box_w/2.0).max((box_w/2.0).min(tr_a.translation.x - tr_b.translation.x)),
                    (-box_h/2.0).max((box_h/2.0).min(tr_a.translation.z - tr_b.translation.z))
                );
                let u = Vec2::new(tr_a.translation.x, tr_a.translation.z) - Vec2::new(tr_b.translation.x, tr_b.translation.z) - diff;
                if u.length() < ball_r {
                    let norm = u.normalize();
                    //println!("{:?} {:?}", u, norm);
                    let res = 2.0 * (ball_r - u.length()) * norm;
                    
                    tr_a.translation.x += res.x;
                    tr_a.translation.z += res.y;

                    po_a.speed = po_a.speed + 2.0 * (po_a.speed.dot(-norm)) * norm;
                }
            },
            Colider::CircleColider(_hole_r) => {
                todo!();
                // BALL vs HOLE
                // let center_distance = ((tr_a.translation.x - tr_b.translation.x).powi(2) + (tr_a.translation.z - tr_b.translation.z).powi(2)).sqrt();
                // if center_distance < ball_r + hole_r {
                //     // hole collision
                // }
            },
        }

    }
}