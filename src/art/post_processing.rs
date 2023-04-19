use bevy::render::{
    camera::{RenderTarget, ScalingMode},
    render_resource::{
        Extent3d, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};

use crate::prelude::*;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera.in_base_set(StartupSet::PreStartup))
            .add_startup_system(spawn_vignette)
            .add_system(vignette_sync)
            .register_type::<VignetteSettings>()
            .add_plugin(Material2dPlugin::<VignetteMaterial>::default());
    }
}

fn vignette_sync(
    mut vignette: Query<(&Handle<VignetteMaterial>, &VignetteSettings)>,
    mut materials: ResMut<Assets<VignetteMaterial>>,
) {
    if let Ok((handle, settings)) = vignette.get_single_mut() {
        let mut current = materials.get_mut(handle).unwrap();
        current.settings = *settings;
    }
}

#[derive(ShaderType, Clone, Copy, Default, Component, Reflect)]
pub struct VignetteSettings {
    pub radius: f32,
    pub feather: f32,
    //XXX wasm builds seem to require binding have size of 16 bytes.......
    pub _holder_for_wasm: f32,
    pub _holder_for_wasm2: f32,
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "129ea159-a4fb-93a1-ab08-54871ea91252"]
pub struct VignetteMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    #[uniform(2)]
    pub settings: VignetteSettings,
}

impl Material2d for VignetteMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vignette.wgsl".into()
    }
}

fn spawn_vignette(
    mut commands: Commands,
    image: Res<MainRender>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VignetteMaterial>>,
) {
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(16.0, 9.0))));

    let vignette_handle = materials.add(VignetteMaterial {
        source_image: image.0.clone(),
        settings: VignetteSettings::default(),
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: vignette_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..default()
            },
            ..default()
        },
        VignetteSettings {
            radius: 1.5,
            feather: 0.6,
            ..default()
        },
        post_processing_pass_layer,
        Name::new("Post Processing Vignette"),
    ));
}

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
