struct EntityData {
    data: i32
}

@group(0) @binding(0) 
var<storage, read_write> entities: array<EntityData>;

@compute @workgroup_size(8)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    return;
}