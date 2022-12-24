use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Obj, Vertex};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub tex_coords: Vec<f32>,
}

/**
 * Implementation of the indexed model.
 */
impl IndexedModel {
    /**
     * Turn a model into its indexed equivalent
     */
    pub fn new(object: &Obj) -> IndexedModel {
        let mut vertex_map: Vec<Vertex> = Vec::new();
        let mut positions: Vec<f32> = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for vertex in object.vertices() {
            match vertex_map
                .iter()
                .position(|check_vertex| check_vertex.position_index() == vertex.position_index())
            {
                Some(index) => indices.push(index as u16),
                None => {
                    // Remember this vertex and store the index
                    indices.push(vertex_map.len() as u16);
                    vertex_map.push(vertex);
                    
                    // Add the position coords
                    positions.push(vertex.position()[0]);
                    positions.push(vertex.position()[1]);
                    positions.push(vertex.position()[2]);
                    
                    // Add the texture coords (we only use the UV/ST coords, no 3D textures).
                    let uv = vertex.uv().unwrap_or([0.0, 0.0, 0.0]);
                    tex_coords.push(uv[0]);
                    tex_coords.push(uv[1]);
                }
            }
        }

        return IndexedModel {
            vertices: positions,
            indices,
            tex_coords,
        };
    }
}
