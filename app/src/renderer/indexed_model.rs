use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Index, Obj, Vertex};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub positions: Vec<f32>,
    pub position_indices: Vec<u16>,
    pub tex_coords: Vec<f32>,
    pub tex_coord_indices: Vec<u16>
}

/**
 * Implementation of the indexed model.
 */
impl IndexedModel {
    /**
     * Turn a model into its indexed equivalent.
     *
     * This is done by filling a memotable whose keys are known vertices that we have seen before,
     * and whose values are indexes into an array containing the position of those vertices.
     */
    pub fn new(obj_data: &Obj) -> IndexedModel {
        let mut vertex_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut tex_coord_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut positions = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        
        // Add vertex positions
        let position_indices = obj_data
            .vertices()
            .map(|vertex| {
                let vertex_id = vertex.position_index();
                *vertex_memo.entry(vertex_id).or_insert_with(|| {
                    let index = (positions.len() / 3) as u16;
                    positions.extend(vertex.position());
                    index
                })
            })
            .collect();

        // Add tex coords
        let tex_coord_indices = obj_data
            .vertices()
            .map(|vertex| {
                let tex_coord_id = vertex.uv_index().unwrap();
                *tex_coord_memo.entry(tex_coord_id).or_insert_with(|| {
                    let index = (tex_coords.len() / 2) as u16;
                    let uvw = vertex.uv().unwrap_or([0.0, 0.0, 0.0]);
                    let mut uv: [f32; 2] = uvw[0..2]
                        .try_into()
                        .expect("UV slice with incorrect length");
                    // Flip the V coord, as it seems to be flipped on the Wii GPU.
                    uv[1] = 1.0 - uv[1];
                    tex_coords.extend(uv);
                    index
                })
            })
            .collect();

        println!("Verts: {:?}, UVs: {:?}, UVidx: {:?}", positions, tex_coords, tex_coord_indices);

        IndexedModel {
            positions,
            position_indices,
            tex_coords,
            tex_coord_indices,
        }
    }
}
