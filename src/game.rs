use hecs::*;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/**
 * Spawn multiple entities in the world
 */
pub fn batch_spawn_entities(world: &mut World, n: i32) {
    for index in 0..n {
        const row_width : i32 = 10;
        let pos_x : f32 = (index % row_width) as f32;
        let pos_z : f32 = (index / row_width) as f32;

        let position = Vector3 {
            x: pos_x,
            y: 0.0,
            z: pos_z,
        };
        let velocity = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        //world.spawn((position, velocity));
    };
}

pub fn system_integrate_motion(
    world: &mut World, 
    query: &mut PreparedQuery<(&mut Vector3, &Vector3)>
) {
    for (_id, (position, velocity)) in query.query_mut(world) {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}