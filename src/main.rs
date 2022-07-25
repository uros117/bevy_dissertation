use bevy::{prelude::*, input::mouse::MouseMotion, render::{camera::{Camera3d}}};

mod skybox;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(skybox::SkyboxPlugin)
        .insert_resource(Msaa {samples: 4})
        .add_startup_system(startup_system)
        .add_system(system)
        .add_system(mouse_motion)
        .add_system(orbit_camera_startup)
        .add_system(camera_switch)
        .add_system_to_stage(CoreStage::PreUpdate, ball_movement)
        .add_system_to_stage(CoreStage::PostUpdate, physiscs_update)
        .run();
    

}




#[derive(Component)]
struct Rotator {
    angle: Vec2,
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
) {
    
    let arena_mesh_handle = meshes.add(Mesh::from(shape::Box {
        max_x: 5.0,
        max_y: 0.0,
        max_z: 5.0,
        min_x:-5.0,
        min_y:-0.5,
        min_z:-5.0,
    }));

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
        max_z: 1.0,
        min_x:-0.5,
        min_y: 0.0,
        min_z:-1.0,
    }));

    // parent cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: arena_mesh_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Rotator { angle: Vec2::ZERO })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .with_children(|parent| {
            // child cube 
            parent.spawn_bundle(PbrBundle {
                    mesh: ball_handle,
                    material: cube_material_handle.clone(),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                })
                .insert_bundle(bevy_mod_picking::PickableBundle::default())
                .insert(BallComponent)
                .insert(PhysicsObject {
                    acc: Vec2::ZERO,
                    max_acc: Vec2::new(1.0, 1.0),
                    speed: Vec2::ZERO,
                    colider: Colider::CircleColider(0.5)
                });
            
            parent.spawn_bundle(PbrBundle {
                    mesh: box_handle.clone(),
                    material: cube_material_handle.clone(),
                    transform: Transform::from_xyz(-2.0, 0.0, 0.0),
                    ..default()
                })
                .insert_bundle(bevy_mod_picking::PickableBundle::default())
                .insert(PhysicsObject {
                    acc: Vec2::ZERO,
                    max_acc: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    colider: Colider::BoxColider(1.0, 2.0)
                });

            parent.spawn_bundle(PbrBundle {
                    mesh: box_handle,
                    material: cube_material_handle.clone(),
                    transform: Transform::from_xyz(4.0, 0.0, 0.0),
                    ..default()
                })
                .insert_bundle(bevy_mod_picking::PickableBundle::default())
                .insert(PhysicsObject {
                    acc: Vec2::ZERO,
                    max_acc: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    colider: Colider::BoxColider(1.0, 2.0)
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


// ARENA
const ARENA_MAX_ANGLE: f32 = 3.14/6.0;
const ARENA_ANG_MOMENTUM: f32 = 0.8;
fn system(
    _commands: Commands,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rotator)>,
) {
    
    query.for_each_mut(|iter| {
        let (mut tran, mut comp) = iter;

        comp.angle.x = (comp.angle.x + match (keyboard.pressed(KeyCode::Up), keyboard.pressed(KeyCode::Down)) {
            (false, false)  => {-ARENA_ANG_MOMENTUM * comp.angle.x * time.delta_seconds()},
            (false, true)   => {ARENA_ANG_MOMENTUM * time.delta_seconds()},
            (true, false)   => {-ARENA_ANG_MOMENTUM * time.delta_seconds()},
            (true, true)    => {0.0},
        }).clamp(-ARENA_MAX_ANGLE, ARENA_MAX_ANGLE);

        comp.angle.y = (comp.angle.y + match (keyboard.pressed(KeyCode::Right), keyboard.pressed(KeyCode::Left)) {
            (false, false)  => {-ARENA_ANG_MOMENTUM * comp.angle.y * time.delta_seconds()},
            (false, true)   => {ARENA_ANG_MOMENTUM * time.delta_seconds()},
            (true, false)   => {-ARENA_ANG_MOMENTUM * time.delta_seconds()},
            (true, true)    => {0.0},
        }).clamp(-ARENA_MAX_ANGLE, ARENA_MAX_ANGLE);

        tran.rotation = Quat::from_rotation_x(comp.angle.x) * Quat::from_rotation_z(comp.angle.y);
    });
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

//https://lib.rs/crates/bevy_mod_picking

// Ball movement
#[derive(Component)]
struct BallComponent;


const SPEED_DAMP:f32 = 1.0;
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

// OBSTACLE
#[derive(Clone, Debug)]
enum Colider {
    BoxColider(f32, f32),
    CircleColider(f32),
}


#[derive(Component, Debug)]
struct PhysicsObject {
    speed: Vec2,
    acc: Vec2,
    max_acc: Vec2,
    colider: Colider,
}



fn physiscs_update(
    mut physics_obj_query: Query<(Entity, &mut Transform, &mut PhysicsObject)>,
) {
    let mut iterator = physics_obj_query.iter_combinations_mut::<2>();
    while let Some(mut arr) = iterator.fetch_next()  {
        let (l, m) = arr.split_at_mut(1);
        if let Some(a) = l.get_mut(0) {
            //(_ent_a, mut tr_a, mut po_a)
            if let Some(b) = m.get_mut(0) {
                //(_ent_b, mut tr_b, mut po_b)
                //print!("{:?} ", a.2);
                let tr_a: &mut Transform = &mut a.1;
                let po_a: &mut PhysicsObject = &mut a.2;
                let tr_b: &mut Transform = &mut b.1;
                let po_b: &mut PhysicsObject = &mut b.2;
                resolve_colission(tr_a, po_a, tr_b, po_b);
                //println!("{:?} ", a.2);
            }
        }
    }
}

fn resolve_colission(tr_a: &mut Transform, po_a: &mut PhysicsObject, tr_b: &mut Transform, po_b: &mut PhysicsObject) {
    match (po_a.colider.clone(), po_b.colider.clone()) {
        (Colider::BoxColider(_w_a, _h_a), Colider::BoxColider(_w_b, _h_b)) => {
            // box vs box
            
        },
        (Colider::BoxColider(_w, _h), Colider::CircleColider(_r)) => {

        },
        (Colider::CircleColider(r_a), Colider::BoxColider(w_b, h_b)) => {
            // box vs circle
            let diff = Vec2::new(
                (-w_b/2.0).max((w_b/2.0).min(tr_a.translation.x - tr_b.translation.x)),
                (-h_b/2.0).max((h_b/2.0).min(tr_a.translation.z - tr_b.translation.z))
            );
            let u = Vec2::new(tr_a.translation.x, tr_a.translation.z) - Vec2::new(tr_b.translation.x, tr_b.translation.z) - diff;
            if u.length() < r_a {
                let norm = u.normalize();
                //println!("{:?} {:?}", u, norm);
                let res = 2.0 * (r_a - u.length()) * norm;
                
                tr_a.translation.x += res.x;
                tr_a.translation.z += res.y;

                po_a.speed = po_a.speed + 2.0 * (po_a.speed.dot(-norm)) * norm;
                //po_a.speed = Vec2::new(if u.x == 0.0 {1.0} else {-2.0} * po_a.speed.x, if u.y == 0.0 {1.0} else {-2.0} * po_a.speed.y);
            }
            
        },
        (Colider::CircleColider(_r_a), Colider::CircleColider(_r_b)) => {
            // circle vs circle

        },
    }
}


// physics_update general
// fn physiscs_update(
//     mut physics_obj_query: Query<(Entity, &mut Transform, &mut PhysicsObject)>,
// ) {
//     let mut iterator = physics_obj_query.iter_combinations_mut::<2>();
//     while let Some(mut arr) = iterator.fetch_next()  {
//         let (l, m) = arr.split_at_mut(1);
//         if let Some(a) = l.get_mut(0) {
//             //(_ent_a, mut tr_a, mut po_a)
//             if let Some(b) = m.get_mut(0) {
//                 //(_ent_b, mut tr_b, mut po_b)
//                 //print!("{:?} ", a.2);
//                 let tr_a: &mut Transform = &mut a.1;
//                 let po_a: &mut PhysicsObject = &mut a.2;
//                 let tr_b: &mut Transform = &mut b.1;
//                 let po_b: &mut PhysicsObject = &mut b.2;
//                 resolve_colission(tr_a, po_a, tr_b, po_b);
//                 //println!("{:?} ", a.2);
//             }
//         }
//     }
// }