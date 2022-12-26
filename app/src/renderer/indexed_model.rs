use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Index, Obj, Vertex};

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
        let mut seen: BTreeMap<Index, u16> = BTreeMap::new();
        let mut vertices = Vec::new();
        let indices = object
            .vertices()
            .map(|vertex| {
                let pos = vertex.position_index();
                match seen.get(&pos) {
                    None => {
                        let index = seen.len() as u16;
                        seen.insert(pos, index);
                        vertices.extend(vertex.position());
                        index
                    }
                    Some(index) => *index,
                }
            })
            .collect();

        IndexedModel { vertices, indices }
    }
}
