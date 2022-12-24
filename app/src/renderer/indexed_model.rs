use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Obj, Vertex};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
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
        let mut indices: Vec<u16> = Vec::new();
        for vertex in object.vertices() {
            match vertex_map
                .iter()
                .position(|check_vertex| check_vertex.position_index() == vertex.position_index())
            {
                Some(index) => indices.push(index as u16),
                None => {
                    indices.push(vertex_map.len() as u16);
                    vertex_map.push(vertex);
                    positions.push(vertex.position()[0]);
                    positions.push(vertex.position()[1]);
                    positions.push(vertex.position()[2]);
                }
            }
        }
        println!("Loaded: {:?}, {:?}", positions, indices);
        return IndexedModel {
            vertices: positions,
            indices,
        };
    }
}
