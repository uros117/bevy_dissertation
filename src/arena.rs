use bevy::{prelude::*, asset::Assets , reflect::TypeUuid, utils::Instant};

use super::GameState;
pub struct ArenaPlugin;

pub const ARENA_MAX_ANGLE: f32 = 3.14/6.0;
pub const ARENA_ANG_MOMENTUM: f32 = 0.8;
pub const ARENA_SIZE: f32 = 12.0;

#[derive(Bundle, Default)]
pub struct ArenaBundle {
    pub rotator: Rotator,
    pub return_anim: ReturnAnimation,
    #[bundle]
    pub pbr: PbrBundle,
}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(system))
            .add_system_set(SystemSet::on_update(GameState::RespawnShrink).with_system(return_to_neutral))
            .add_asset::<ArenaAssets>()
            .init_resource::<ArenaAssets>();
    }
}

#[derive(TypeUuid)]
#[uuid = "9e062dfb-8484-415c-86e0-28cd451874d8"]
pub struct ArenaAssets {
    pub mesh: Handle<Mesh>,
    pub tex: Handle<Image>,
}

impl FromWorld for ArenaAssets {
    fn from_world(world: &mut World) -> Self {
        let arena_mesh_handle = world.resource_mut::<Assets<Mesh>>().add(
            Mesh::from(shape::Box {
                max_x: ARENA_SIZE / 2.0,
                max_y: 0.0,
                max_z: ARENA_SIZE / 2.0,
                min_x:-ARENA_SIZE / 2.0,
                min_y:-0.5,
                min_z:-ARENA_SIZE / 2.0,
            }));

        let tex_handle = world.resource::<AssetServer>().load("wood.png");
        
        ArenaAssets { mesh: arena_mesh_handle, tex: tex_handle }
    }
}

#[derive(Component, Default)]
pub struct Rotator {
    pub angle: Vec2,
}

#[derive(Component, Default)]
pub struct ReturnAnimation {
    start_time: Option<Instant>,
    start: Vec2,
}


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

fn return_to_neutral(
    _commands: Commands,
    mut query: Query<(&mut Transform, &mut Rotator, &mut ReturnAnimation)>,
) {
    query.for_each_mut(|iter| {
        let (mut tran, mut comp, mut anim) = iter;

        if let None = anim.start_time {
            anim.start_time = Some(Instant::now());
            anim.start = comp.angle;
        }

        if let Some(start_instant) = anim.start_time {
            let delta = Instant::now() - start_instant;
            
            comp.angle = (1.0 - delta.as_secs_f32()).clamp(0.0, 1.0) * anim.start;

            if delta.as_secs_f32() >= 1.0 {
                anim.start_time = None;
            }
        }

        tran.rotation = Quat::from_rotation_x(comp.angle.x) * Quat::from_rotation_z(comp.angle.y);
    });
}

