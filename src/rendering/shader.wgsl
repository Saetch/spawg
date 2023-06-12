// Vertex shader, this is code that tells the GPU how to draw the vertices of a shape

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}


// Fragment shader, this is code that tells the GPU how to draw the pixels contained within a shape

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {  //<-- location(0) means first color target of the pixel
    return vec4<f32>(in.color, 1.0);
}

