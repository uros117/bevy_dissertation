use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::GameState;

use crate::physics::*;
use crate::ball::*;

#[derive(Component, Default)]
pub struct HoleComponent {
    pub is_final: bool,
} 

#[derive(TypeUuid)]
#[uuid = "ad0913f1-1770-45f2-ab88-cfbcb9ae67f5"]
pub struct HoleAssets {
    pub mesh: Handle<Mesh>,
    pub tex: Handle<Image>,
    pub final_tex: Handle<Image>,
    pub hole_material_handle: Handle<StandardMaterial>,
    pub final_hole_material_handle: Handle<StandardMaterial>,
}

impl FromWorld for HoleAssets {
    fn from_world(world: &mut World) -> Self {

        // world.resource_scope(|world, mut res:Mut<HoleAssets>| {

        // });

        let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(
        Mesh::from(
            shape::Plane {
                size: 1.5//Vec2::new(1.0, 1.0),
            }));

        let tex_handle = world.resource::<AssetServer>().load("hole.png");
        let final_tex_handle = world.resource::<AssetServer>().load("final_hole.png");

        let hole_material_handle = world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial { 
            //base_color: Color::RED, 
            base_color_texture: Some(tex_handle.clone()),
            metallic: 0.0,
            reflectance: 0.0,
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        });

        let final_hole_material_handle = world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial { 
            //base_color: Color::RED, 
            base_color_texture: Some(final_tex_handle.clone()),
            metallic: 0.0,
            reflectance: 0.0,
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        });

        HoleAssets { 
            mesh: mesh_handle, 
            tex: tex_handle, 
            final_tex: final_tex_handle,
            hole_material_handle: hole_material_handle,
            final_hole_material_handle: final_hole_material_handle,
        }
    }
}

#[derive(Bundle)]
pub struct HoleBundle {
    pub hole_comp: HoleComponent,
    pub po: PhysicsObject,
    #[bundle]
    pub pbr: PbrBundle,
}

impl Default for HoleBundle {
    fn default() -> Self {
        Self { 
            hole_comp: Default::default(), 
            po: 
                PhysicsObject {
                    colider: Colider::CircleColider(0.15),
                    ..default()
                }, 
            pbr: Default::default() 
        }
    }
}

// Tried implementing new function to construct a new HoleBundle
// FromWorld could have been used
// Problem was that world is not accessable in normal system, only in exclusive ones
//
// impl HoleBundle {
//     pub fn new(world: &World, transform: Transform, is_final: bool) -> Self {
//         let hole_assets = world.get_resource::<HoleAssets>().unwrap();
//         Self { 
//             hole_comp: 
//                 Default::default(), 
//             po: 
//                 PhysicsObject {
//                     colider: Colider::CircleColider(0.15),
//                     ..default()
//                 }, 
//             pbr: 
//                 PbrBundle {
//                     mesh: hole_assets.mesh.clone(),
//                     material: if is_final { hole_assets.final_hole_material_handle.clone() } else { hole_assets.hole_material_handle.clone() },
//                     transform: transform,
//                     ..default()
//                 }

//         }
//     }
// }

pub struct HolePlugin;

impl Plugin for HolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(hole_system))
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