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

struct Constants {
    time_offset : f32,
    dissolve_speed : f32,
};

@group(1) @binding(0)
var<uniform> constants : Constants;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn main_fragment(input: VertexOutput) -> @location(0) vec4<f32> {

    // var pi: f32 = 3.1415;

    var r: f32 = 0.2;
    var timed_radius = r * constants.time_offset; 

    var center: vec2<f32> = vec2<f32>(1.0, 1.0);
    var point: vec2<f32> = vec2<f32>(input.tex_coords.x, input.tex_coords.y);
    var texture_color : vec4<f32> = textureSample(t_diffuse, s_diffuse, input.tex_coords);


    if (in_circle(center, point, timed_radius)) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } else {
        return texture_color;
    }

}

fn in_circle(center: vec2<f32>, point: vec2<f32>, radius: f32) -> bool {
    var d = sqrt( pow(point.x - center.x, 2.0) + pow(point.y - center.y, 2.0));
    return d < radius;
}