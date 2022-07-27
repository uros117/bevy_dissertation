use bevy::{prelude::*, input::mouse::MouseMotion, render::{camera::{Camera3d}}};

mod skybox;
mod obstacle;
mod physics;
mod arena;
mod ball;
mod hole;

use physics::*;
use arena::*;
use ball::*;
use obstacle::*;
use hole::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(skybox::SkyboxPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ObstaclePlugin)
        .add_plugin(HolePlugin)
        .insert_resource(Msaa {samples: 4})
        .add_startup_system(startup_system)
        .add_system(mouse_motion)
        .add_system(orbit_camera_startup)
        .add_system(camera_switch)
        .run();
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
) {
    
    let cube_material_handle = materials.add(StandardMaterial { 
        base_color: Color::RED, 
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

    // let hole_mesh_handle = meshes.add(Mesh::from(shape::Capsule {
    //     radius: todo!(),
    //     rings: todo!(),
    //     depth: todo!(),
    //     latitudes: todo!(),
    //     longitudes: todo!(),
    //     uv_profile: todo!(),
    // }));

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
            parent.spawn_bundle(
                BallBundle {
                    ball_comp: BallComponent,
                    po: PhysicsObject {
                        acc: Vec2::ZERO,
                        max_acc: Vec2::new(1.0, 1.0),
                        speed: Vec2::ZERO,
                        colider: physics::Colider::CircleColider(0.5)
                    },
                    pbr: PbrBundle {
                        mesh: ball_handle,
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
                        colider: physics::Colider::BoxColider(1.0, 2.0)
                    },
                    pbr: PbrBundle {
                        mesh: box_handle.clone(),
                        material: cube_material_handle.clone(),
                        transform: Transform::from_xyz(-2.0, 0.0, 0.0)
                            .with_scale(Vec3::new(1.0, 1.0, 2.0)),
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
        });
    
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform { rotation: Quat::from_rotation_x(- std::f32::consts::PI / 4.0), ..default()},
            ..default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(MovableCamera{
            ..default()
        });
    
    let mut ortho_camera = OrthographicCameraBundle::new_3d();
    //ortho_camera.orthographic_projection
    /*ortho_camera.transform = Transform {
        translation: Vec3::new(0.0, 10.0, 0.0),
        rotation: Quat::from_rotation_z(- std::f32::consts::PI),
        ..default()
    };*/
    
    ortho_camera.transform.rotate(Quat::from_rotation_x(- std::f32::consts::PI / 2.0));
    ortho_camera.orthographic_projection.scale = 4.0;
    ortho_camera.transform.translation += Vec3::new(0.0, 2.0, 0.0);

    commands.spawn_bundle(ortho_camera);

}

// CAMERA SWITCH
fn camera_switch(
    _commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut active_cameras: ResMut<bevy::render::camera::ActiveCamera<Camera3d>>,
    ortho_camera_query: Query<(Entity, &OrthographicProjection, &Camera)>,
    persp_camera_query: Query<(Entity, &PerspectiveProjection, &Camera)>,
) {
    if keyboard.just_pressed(KeyCode::Numpad2) {
        active_cameras.set(ortho_camera_query.single().0);
    } else if keyboard.just_pressed(KeyCode::Numpad1) {
        active_cameras.set(persp_camera_query.single().0);
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
