use bevy::{prelude::*, ecs::system::lifetimeless::SRes, render::{renderer::RenderDevice, render_resource::{BindGroupLayout, BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, ShaderStages, TextureViewDimension, TextureSampleType, BindingType, BindGroupLayoutEntry, SamplerBindingType, BindGroupEntry}, render_asset::{RenderAsset, RenderAssets, PrepareAssetError}, mesh::Indices}, reflect::TypeUuid, pbr::MaterialPipeline};


pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<SkyboxMaterial>::default())
            .add_startup_system(startup_system);
    }
}


#[derive(TypeUuid, Clone)]
#[uuid = "c7df202e-8e36-4a43-b195-e17bba95ed93"]
pub struct SkyboxMaterial {
    cubemap: Option<Handle<Image>>,
}

// Asset is implemented for any type that has a trait TypeUUID
// so implementing TypeUUID is enogh

pub struct SkyboxMaterialGPU {
    bind_group: BindGroup,
}


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
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("shaders/skybox_fragment.wgsl"))
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("shaders/skybox_vertex.wgsl"))
    }

    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        // Bind groups are used to setup uniforms
        // This function is called every frame
        // and bind_group doesn't need to be created
        // every frame
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },// filterable means that the image can be interpolated
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,// None because this is not an array
                }
            ],
        })
    }
}

// Transfer the material from the App world to the Render world(there 2 seperate ECS worlds in a project)
// Extracted asset can be the same type as the render asset
impl RenderAsset for SkyboxMaterial {
    type ExtractedAsset = SkyboxMaterial;

    type PreparedAsset = SkyboxMaterialGPU;

    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<SkyboxMaterial>>, SRes<RenderAssets<Image>>);// There are no parameters needed at the time

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline, gpu_images): &mut bevy::ecs::system::SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>> {

        let (base_color_texture_view, base_color_sampler) = 
            if let Some(result) = pipeline.mesh_pipeline.get_image_texture(gpu_images, &extracted_asset.cubemap) {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: bevy::render::render_resource::BindingResource::TextureView(base_color_texture_view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: bevy::render::render_resource::BindingResource::Sampler(base_color_sampler)
                }
            ],
        });
        Ok(SkyboxMaterialGPU {
            bind_group,
        })
    }
}