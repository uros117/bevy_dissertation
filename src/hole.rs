use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::GameState;

use crate::physics::*;
use crate::ball::*;

#[derive(Component)]
pub struct HoleComponent {
    pub is_final: bool,
} 

#[derive(TypeUuid)]
#[uuid = "ad0913f1-1770-45f2-ab88-cfbcb9ae67f5"]
pub struct HoleAssets {
    pub mesh: Handle<Mesh>,
    pub tex: Handle<Image>,
    pub final_tex: Handle<Image>,
}

impl FromWorld for HoleAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(
        Mesh::from(
            shape::Plane {
                size: 1.5//Vec2::new(1.0, 1.0),
            }));

        let tex_handle = world.resource::<AssetServer>().load("hole.png");
        let final_tex_handle = world.resource::<AssetServer>().load("final_hole.png");

        HoleAssets { mesh: mesh_handle, tex: tex_handle, final_tex: final_tex_handle }
    }
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
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(hole_system))
            .add_asset::<HoleAssets>()
            .init_resource::<HoleAssets>();
    }
}

fn hole_system(
    mut game_state: ResMut<State<GameState>>,
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
                let po_a = ball.1.as_mut();
                let _ball_component = ball.2;
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
                                game_state.set(GameState::Splash).unwrap();

                            } else {
                                // is not a final
                                reset_ball(&mut game_state);
                                return;
                            }

                        }
                    }
                }
            }

        }
    }
}