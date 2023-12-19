@group(0) @binding(0)
var<uniform> time: f32;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) p: vec2<f32>
};

fn absolute(x: f32) -> f32 {
    //var out: f32;
    if x < 0.0 { return -x; } else { return x; }
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    //let x = f32(1 - i32(in_vertex_index)) * 0.5;
    //let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    //out.clip_position = vec4<f32>(x, y + sin(time) * 0.5, 0.0, 1.0);
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.clip_position.y += sin(time) * 0.5;
    out.p = model.position.xy;
    
   // let center = vec2<f32>(0.0, -1.0 / 6.0);
    //out.distance = (4.0 * model.position.x);// 0.5 - abs(model.position.x);
    //max(0.0, length(model.position.xy - center) * 0.5);// * 6.0 / sqrt(13.0);
    return out;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    /*let p = in.clip_position;
    let r = p.x;
    let g = p.y;
    let b = 0.0;*/
    let center = vec2<f32>(0.0, -1.0 / 6.0);
    let distance = min(length(in.p - center) * 2.0, 1.0);
    let alpha = 0.5 + sin(time + 3.14159 * 1.0) * 0.5;
    return vec4<f32>(in.color, (1.0 - pow(distance, 0.25)));
}