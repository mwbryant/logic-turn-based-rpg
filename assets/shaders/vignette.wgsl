#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils
//#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct Settings {
    radius: f32,
    feather: f32,
    holder1: f32,
    holder2: f32,
}

@group(1) @binding(2)
var<uniform> settings: Settings;


@fragment
fn fragment(
    // Gives us uv
    // https://github.com/bevyengine/bevy/blob/v0.10.1/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let centered_uv = uv * 2.0 - 1.0;

    let circle = length(centered_uv);

    let mask = 1.0 - smoothstep(settings.radius - settings.feather, settings.radius, circle);

    let color = textureSample(texture, our_sampler, uv);
        //textureSample(texture, our_sampler, sample_uv).bgr * mix;

    var output_color = vec4<f32>(
        color.rgb * mask,
        //(sin(globals.time*1.123 + 503.2523) * 0.15 + 0.9)
        1.0
    );

    return output_color;
}