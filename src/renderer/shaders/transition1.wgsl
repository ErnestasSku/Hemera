
//////// VERTEX

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn main_vertex(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

///////// FRAGMENT

struct Constants {
    time_offset : f32,
    dissolve_speed : f32,
};

// fn random_coord(vec2 co) -> f32 {
//     return fract(sin(dot(co.xy, vec2(12.9898, 96.233))) * 43758.5453);
// }

// [[group(0), binding(0)]] 
@group(1) @binding(0)
var<uniform> constants : Constants;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

// Helper function to generate a random number
fn random_coord(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2(12.9898, 96.233))) * 43758.5453);
}


@fragment
fn main_fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var TEXTURE_PIXEL_SIZE: vec2<f32> = vec2(1.0 / 128.0, 1.0 / 128.0);
    var texture_resolution : vec2<f32> = vec2<f32>(1.0, 1.0) / TEXTURE_PIXEL_SIZE;
    // var texture_resolution : TEXTURE_PIXEL_SIZE;
    var pixel_within_texture : vec2<f32> = floor(input.tex_coords * texture_resolution);
    var texture_color : vec4<f32> = textureSample(t_diffuse, s_diffuse, input.tex_coords);

    if (sin((constants.time_offset * constants.dissolve_speed) + 0.0) < random_coord(pixel_within_texture)) {
        // Set to the original texture color
        return texture_color;
    }
    else {
        // German flag
        if (input.tex_coords.y < 0.33) {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
        else if (input.tex_coords.y < 0.66) {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
        else {
            return vec4<f32>(1.0, 1.0, 0.0, 1.0);
        }
        // return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
}
