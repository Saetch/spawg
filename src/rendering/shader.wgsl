// Vertex shader, this is code that tells the GPU how to draw the vertices of a shape

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) sprite_position: vec2<f32>,
    @location(3) tex_i: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tex_i: u32

};

struct Camera {
    position: vec2<f32>,
    size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> cam: Camera;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    var semi_size = cam.size / 2.0;
    out.clip_position = vec4<f32>(model.position /semi_size + (instance.sprite_position - cam.position)/ semi_size, 0.0, 1.0);
    out.tex_i = instance.tex_i;
    return out;
}

@group(0) @binding(0)
var t_diffuse: binding_array<texture_2d<f32>>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let image = t_diffuse[in.tex_i];
    return textureSample(image, s_diffuse, in.tex_coords);
}