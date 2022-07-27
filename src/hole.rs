use bevy::prelude::*;
use bevy::app::AppExit;

use crate::physics::*;
use crate::ball::*;

#[derive(Component)]
pub struct HoleComponent {
    is_final: bool,
} 

#[derive(Bundle)]
pub struct HoleBundle {
    pub hole_comp: HoleComponent,
    pub po: PhysicsObject,
    #[bundle]
    pub pbr: PbrBundle,
}

pub struct HolePlugin;

impl Plugin for HolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::PostUpdate, hole_system);
    }
}

fn hole_system(
    mut exit: EventWriter<AppExit>,
    mut ball_query: Query<(&mut Transform, &mut PhysicsObject, &BallComponent)>,
    mut hole_query: Query<(&mut Transform, &mut PhysicsObject, &HoleComponent, Without<BallComponent>)>,
) {
    let mut ball_iterator = ball_query.iter_mut();
    // Grab all ball entitys
    while let Some(mut ball) = ball_iterator.next()  {
        let mut other_iter = hole_query.iter_mut();
        // Grab other coliders
        while let Some(other) = other_iter.next()  {

            {
                let tr_a = ball.0.as_mut();
                let po_a = ball.1.as_ref();
                let tr_b = other.0;
                let po_b = other.1;
                let hc = other.2;
                if let Colider::CircleColider(ball_r) = po_a.colider {
                    if let Colider::CircleColider(hole_r) = po_b.colider {
                        // BALL vs HOLE
                        let center_distance = ((tr_a.translation.x - tr_b.translation.x).powi(2) + (tr_a.translation.z - tr_b.translation.z).powi(2)).sqrt();
                        if center_distance < ball_r + hole_r {
                            // hole collision
                            if hc.is_final {
                                // is a final hole
                                exit.send(AppExit);

                            } else {
                                // is not a final
                                tr_a.translation = Vec3::ZERO;
                            }

                        }
                    }
                }
            }

        }
    }
}