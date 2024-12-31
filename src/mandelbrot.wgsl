struct Uniform {
    bounds: vec4<f32>,
    max_iterations: u32,
}

@group(0)
@binding(0)
var texture: texture_storage_2d<r32uint, write>;

@group(0)
@binding(1)
var<uniform> args: Uniform;

@compute
@workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    var bounds = args.bounds;
    var max_iterations = args.max_iterations;

    var final_iteration = max_iterations;
    var real_step = (bounds.x - bounds.y) / f32(textureDimensions(texture).x);
    var imag_step = (bounds.z - bounds.w) / f32(textureDimensions(texture).y);

    var c = vec2(
        // Translated to put everything nicely in frame.
        bounds.x + (f32(id.x) * real_step),
        bounds.z + (f32(id.y) * imag_step),
    );

    var current_z = c;
    var next_z: vec2<f32>;
    for (var i = 0u; i < max_iterations; i++) {
        next_z.x = (current_z.x * current_z.x - current_z.y * current_z.y) + c.x;
        next_z.y = (2.0 * current_z.x * current_z.y) + c.y;
        current_z = next_z;
        if length(current_z) > 4.0 {
            final_iteration = i;
            break;
        }
    }
    textureStore(texture, vec2(i32(id.x), i32(id.y)), final_iteration);
}
