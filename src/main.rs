use std::time::Duration;

use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}, render::{camera::{Camera3d}}};

use bevy_tweening::{*, lens::TransformScaleLens};

mod skybox;
mod obstacle;
mod physics;
mod arena;
mod ball;
mod hole;
mod ball_anim;
mod splash;

use physics::*;
use arena::*;
use ball::*;
use obstacle::*;
use hole::*;
use ball_anim::*;
use splash::*;

fn main() {
    App::new()
        .add_state(GameState::RespawnGrow)
        .add_plugins(DefaultPlugins)
        .add_plugin(skybox::SkyboxPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(HolePlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(BallAnimPlugin)
        .add_plugin(SplashPlugin)
        .insert_resource(Msaa {samples: 4})
        .add_startup_system(startup_system)
        .add_system(mouse_motion)
        .add_system(orbit_camera_startup)
        .add_system(camera_switch)
        .add_system(move_top_down_camera)
        .add_system(mouse_scroll)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    RespawnGrow,
    Running,
    RespawnShrink,
    Splash,
}

#[derive(Component)]
struct MovableCamera {
    focus_distance: f32,
    focus: Vec3,
}

impl Default for MovableCamera {
    fn default() -> Self {
        MovableCamera{
            focus_distance: 20.0,
            focus: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
struct TopDownCamera {
    focus_distance: f32,
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            focus_distance: 5.0,
        }
    }
}

// Accessing resources using Res/ResMut
// Accessing components of entities using queries(Query)
// Creating/destroying entities, components, and resources using Commands(Commands)
// Sending/receiving events using EventWriter/EventReader
fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    arena_assets: Res<ArenaAssets>,
    hole_assets: Res<HoleAssets>,
) {
    println!("Settingup the stage!");
    
    let cube_material_handle = materials.add(StandardMaterial { 
        //base_color: Color::RED, 
        base_color_texture: Some(arena_assets.tex.clone()),
        metallic: 0.0,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        ..default()
    });

    let hole_material_handle = materials.add(StandardMaterial { 
        //base_color: Color::RED, 
        base_color_texture: Some(hole_assets.tex.clone()),
        metallic: 0.0,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });

    let final_hole_material_handle = materials.add(StandardMaterial { 
        //base_color: Color::RED, 
        base_color_texture: Some(hole_assets.final_tex.clone()),
        metallic: 0.0,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });

    let ball_material_handle = materials.add(StandardMaterial { 
        base_color: Color::rgb_u8(200, 200, 200), 
        metallic: 0.7,
        ..default()
    });

    let ball_handle = meshes.add(Mesh::from(shape::UVSphere{
        radius: 0.5,
        ..default()
    }));

    let box_handle = meshes.add(Mesh::from(shape::Box{
        max_x: 0.5,
        max_y: 1.0,
        max_z: 0.5,
        min_x:-0.5,
        min_y: 0.0,
        min_z:-0.5,
    }));

    // parent cube
    commands
        .spawn_bundle(ArenaBundle {
            rotator: Rotator { angle: Vec2::ZERO },
            pbr: PbrBundle {
                mesh: arena_assets.mesh.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        })
        .with_children(|parent| {
            // child cube 
            // BALL
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_secs(2),
                TransformScaleLens {
                    start: Vec3::new(1.0, 1.0, 1.0),
                    end: Vec3::new(0.1, 0.1, 0.1),
                },
            )
            .with_completed_event(true, 1);

            let mut anim = Animator::new(tween);
            anim.stop();

            parent.spawn_bundle(
                BallBundle {
                    ball_comp: BallComponent {
                        start_pos: Transform::from_xyz(-5.0, 0.5, -5.0),
                    },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.5)
                    },
                    pbr: PbrBundle {
                        mesh: ball_handle,
                        material: ball_material_handle.clone(),
                        transform: Transform::from_xyz(-5.0, 0.5, -5.0),
                        ..default()
                    }
                })
                .insert(anim);
            
            // HOLES
            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: true },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: final_hole_material_handle.clone(),
                        transform: Transform::from_xyz(5.0, 0.001, 5.0),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(3.0, 0.001, -3.0),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, 0.001, -2.8),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, 0.001, -2.8),
                        ..default()
                    }
                });


            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(0.0, 0.001, 1.8),
                        ..default()
                    }
                });


            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(-1.5, 0.001, 3.8),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                HoleBundle {
                    hole_comp: HoleComponent { is_final: false },
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.15)
                    },
                    pbr: PbrBundle {
                        mesh: hole_assets.mesh.clone(),
                        material: hole_material_handle.clone(),
                        transform: Transform::from_xyz(2.6, 0.001, 4.2),
                        ..default()
                    }
                });
            // OBSTACLES
            parent.spawn_bundle(
                ObstacleBundle {
                    obstacle_comp: ObstacleComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::ZERO,
                        speed: Vec2::ZERO,
                        colider: physics::Colider::BoxColider(0.4, 8.0)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(-5.8, 0.0, 0.0)
                            .with_scale(Vec3::new(0.4, 1.0, 8.0)),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                ObstacleBundle {
                    obstacle_comp: ObstacleComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::ZERO,
                        speed: Vec2::ZERO,
                        colider: physics::Colider::BoxColider(0.4, 1.5)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, 0.0, -4.5)
                            .with_scale(Vec3::new(0.4, 1.0, 1.5)),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                ObstacleBundle {
                    obstacle_comp: ObstacleComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::ZERO,
                        speed: Vec2::ZERO,
                        colider: physics::Colider::BoxColider(0.4, 6.0)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(-3.5, 0.0, 2.5)
                            .with_scale(Vec3::new(0.4, 1.0, 6.0)),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                ObstacleBundle {
                    obstacle_comp: ObstacleComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::ZERO,
                        speed: Vec2::ZERO,
                        colider: physics::Colider::BoxColider(1.0, 4.0)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(4.0, 0.0, 0.0)
                            .with_scale(Vec3::new(1.0, 1.0, 4.0)),
                        ..default()
                    }
                });

            parent.spawn_bundle(
                ObstacleBundle {
                    obstacle_comp: ObstacleComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::ZERO,
                        speed: Vec2::ZERO,
                        colider: physics::Colider::BoxColider(4.0, 0.4)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(1.5, 0.0, 0.0)
                            .with_scale(Vec3::new(4.0, 1.0, 0.4)),
                        ..default()
                    }
                });
        });
    
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, -3.0),
        point_light: PointLight {shadows_enabled: true, ..default()},
        ..default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform { rotation: Quat::from_rotation_x(- std::f32::consts::PI / 4.0), ..default()},
            ..default()
        })
        .insert(MovableCamera{
            ..default()
        });
    
    let mut ortho_camera = OrthographicCameraBundle::new_3d();
    
    ortho_camera.transform.rotate(Quat::from_rotation_x(- std::f32::consts::PI / 2.0));
    ortho_camera.orthographic_projection.scale = 4.0;
    ortho_camera.transform.translation += Vec3::new(0.0, 2.0, 0.0);

    commands.spawn_bundle(ortho_camera).insert(TopDownCamera { focus_distance: 2.0});

    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
}

// CAMERA SWITCH
fn camera_switch(
    _commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut active_cameras: ResMut<bevy::render::camera::ActiveCamera<Camera3d>>,
    ortho_camera_query: Query<(Entity, &OrthographicProjection, &Camera, With<TopDownCamera>)>,
    persp_camera_query: Query<(Entity, &PerspectiveProjection, &Camera)>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        if active_cameras.get().unwrap() == persp_camera_query.single().0 {
            active_cameras.set(ortho_camera_query.single().0);
        } else {
            active_cameras.set(persp_camera_query.single().0);
        }
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut MovableCamera)>,
) {
    if buttons.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            move_orbit_camera(ev.delta, &mut query);
        }
    }
}

fn mouse_scroll(
    mut mouse_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut MovableCamera)>,
) {

    for ev in mouse_evr.iter() {
        for (_cam_transform, mut cam_comp) in query.iter_mut() {
            cam_comp.focus_distance += ev.y * 0.1;
        }
    }
}


fn orbit_camera_startup(
    mut query: Query<(&mut Transform, &mut MovableCamera)>,
) {
    move_orbit_camera(Vec2::ZERO, &mut query);
}

fn move_orbit_camera(
    delta: Vec2,
    query: &mut Query<(&mut Transform, &mut MovableCamera)>
){
    for (mut camera, mc) in query.iter_mut() {
        camera.translation = Vec3::ZERO;
        camera.rotation = camera.rotation * Quat::from_rotation_x(delta.y * 0.001);
        camera.rotation = Quat::from_rotation_y(delta.x * 0.001) * camera.rotation;
        camera.translation = mc.focus + (-mc.focus_distance) * camera.forward();
    }
}

fn move_top_down_camera(
    ball_query: Query<(&GlobalTransform, &BallComponent)>,
    mut query: Query<(&mut Transform, &mut TopDownCamera)>,
) {
    let (ball_trasform, _ball_comp) = ball_query.single();
    for (mut camera, camera_comp) in query.iter_mut() {
        camera.translation.x = ball_trasform.translation.x;
        camera.translation.z = ball_trasform.translation.z;
        camera.translation.y = camera_comp.focus_distance + ball_trasform.translation.y;
    }
}
