use bevy::{prelude::*, render::{ render_resource::{AsBindGroup, ShaderRef}, mesh::Indices}, reflect::TypeUuid};


pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<SkyboxMaterial>::default())
            .add_startup_system(startup_system);
    }
}


#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "c7df202e-8e36-4a43-b195-e17bba95ed93"]
pub struct SkyboxMaterial {
    #[texture(0)]
    #[sampler(1)]
    cubemap: Option<Handle<Image>>,
}

// Asset is implemented for any type that has a trait TypeUUID
// so implementing TypeUUID is enogh

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    mut skybox_materials: ResMut<Assets<SkyboxMaterial>>,
) {
    let skybox_texture = asset_server.load("skybox.png");

    println!("test");
    commands
    .spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(build_skybox_mesh().clone()),
        material: skybox_materials.add(SkyboxMaterial {
            cubemap: Some(skybox_texture.clone())
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

}

const SKYBOX_SCALE:f32 = 20.0;
fn build_skybox_mesh() -> Mesh {
    //
    //             7---6
    //             |+Y |
    //         4---3---2---A---C
    //         |-X |-Z |+X |+Z |
    //         5---0---1---B---D
    //             |-Y |
    //             9---8
    //
    //  4 == 7 == C
    //  5 == 9 == D
    //  6 == A
    //  8 == B

    let mut cubemap_mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);

    cubemap_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        vec![
            [-0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE],// 0 // front
            [ 0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE],// 1
            [ 0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE],// 2
            [-0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE],// 3

            [-0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 4 // left
            [-0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 5
            
            [ 0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 6 // up
            [-0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 7

            [ 0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 8 // down
            [-0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// 9

            [ 0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// A // right
            [ 0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// B

            [-0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// C // back
            [-0.5 * SKYBOX_SCALE, -0.5 * SKYBOX_SCALE,  0.5 * SKYBOX_SCALE],// D
        ]);
    cubemap_mesh.set_indices(Some(Indices::U32(vec![
              0,  1,  2,  //front
              0,  2,  3,
 
              5,  0,  3,  //left
              5,  3,  4,
 
              3,  2,  6,  //up
              3,  6,  7,
 
              9,  1,  0,  //down
              8,  1,  9,
 
              1, 11, 10,  //right
              1, 10,  2,
 
             11, 12, 10,  //back
             11, 13, 12,

    ])));

    cubemap_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0, 
        vec![
            [   0.25    , 0.6666  ],
			[   0.5     , 0.6666  ],
			[   0.5     , 0.3333  ],
			[   0.25    , 0.3333  ],

			[   0.0     , 0.3333  ],
			[   0.0     , 0.6666  ],

			[   0.5     , 0.0     ],
			[   0.25    , 0.0     ],

			[   0.5     , 1.0     ],
			[   0.25    , 1.0     ],

			[   0.75    , 0.3333  ],
			[   0.75    , 0.6666  ],

			[   1.0     , 0.3333  ],
			[   1.0     , 0.6666  ],
        ]);
    
    cubemap_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,  vec![
            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],

            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],

            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],

            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],

            [0.0,  0.0,  0.0],
            [0.0,  0.0,  0.0],
        ]);
    cubemap_mesh
}


impl Material for SkyboxMaterial {
    // the vertex and fragment shaders are optional
    // they use a default "mesh pipeline shader" if they are not defined
    fn fragment_shader() -> ShaderRef {
        "shaders/skybox_fragment.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/skybox_vertex.wgsl".into()
    }
}