use bevy::{prelude::*, asset::Assets , reflect::TypeUuid};

pub struct ArenaPlugin;

pub const ARENA_MAX_ANGLE: f32 = 3.14/6.0;
pub const ARENA_ANG_MOMENTUM: f32 = 0.8;

#[derive(Bundle)]
pub struct ArenaBundle {
    pub rotator: Rotator,
    #[bundle]
    pub pbr: PbrBundle,
}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(system)
            .add_asset::<ArenaAssets>()
            .init_resource::<ArenaAssets>();
    }
}

#[derive(TypeUuid)]
#[uuid = "9e062dfb-8484-415c-86e0-28cd451874d8"]
pub struct ArenaAssets {
    pub mesh: Handle<Mesh>
}

impl FromWorld for ArenaAssets {
    fn from_world(world: &mut World) -> Self {
        let arena_mesh_handle = world.resource_mut::<Assets<Mesh>>().add(
            Mesh::from(shape::Box {
                max_x: 5.0,
                max_y: 0.0,
                max_z: 5.0,
                min_x:-5.0,
                min_y:-0.5,
                min_z:-5.0,
            }));
            
        ArenaAssets { mesh: arena_mesh_handle }
    }
}

#[derive(Component)]
pub struct Rotator {
    pub angle: Vec2,
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

