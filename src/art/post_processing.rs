use bevy::{
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};

use crate::prelude::*;

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(
    mut commands: Commands,
    //windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //let mut camera = Camera2dBundle::default();
    //let size = 10.0;

    //camera.projection.scaling_mode = ScalingMode::FixedVertical(size);

    //commands.spawn(camera);

    //let window = windows.single();

    let size = Extent3d {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        ..default()
    };

    let mut render_target = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    render_target.resize(size);

    let render_image_handle = images.add(render_target);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(10.0);
    //camera.transform.translation.x = 320.0;
    //camera.transform.translation.y = 240.0;
    camera.camera.target = RenderTarget::Image(render_image_handle.clone());

    commands.spawn((
        camera, MainCamera,
        //UiCameraConfig { show_ui: false }
    ));

    commands.insert_resource(MainRender(render_image_handle.clone()));

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(16.0, 9.0))));

    let material_handle = materials.add(ColorMaterial {
        texture: Some(render_image_handle),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        //PostProcessingQuad,
        post_processing_pass_layer,
        Name::new("Base Render"),
    ));
    let mut camera = Camera2dBundle::default();
    camera.camera.order = 999;
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 16.0,
        min_height: 9.0,
    };

    commands.spawn((
        camera,
        post_processing_pass_layer,
        UiCameraConfig { show_ui: false },
    ));
}
