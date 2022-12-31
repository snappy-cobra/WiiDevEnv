use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Index, Obj, Vertex};

/**
 * Our representation of a model.
 */
#[derive(Debug)]
pub struct IndexedModel {
    pub positions: Vec<f32>,
    pub position_indices: Vec<u16>,
    pub tex_coords: Vec<f32>,
    pub tex_coord_indices: Vec<u16>
}

pub const SIZE_POSITION: usize = 3;
pub const BYTE_SIZE_POSITION: usize = core::mem::size_of::<f32>() * SIZE_POSITION;
pub const SIZE_TEX_COORD: usize = 2;
pub const BYTE_SIZE_TEX_COORD: usize = core::mem::size_of::<f32>() * SIZE_TEX_COORD;

/**
 * Implementation of the indexed model.
 * This is done by filling memo tables whose keys are known indexable vertex attributes that we have seen before,
 * and whose values are indexes into an array containing these vertex attributes.
 */
impl IndexedModel {
    /**
     * Turn a model into its indexed equivalent.
     */
    pub fn new(obj_data: &Obj) -> IndexedModel {
        let (positions, position_indices) = Self::index_positions(obj_data);
        let (tex_coords, tex_coord_indices) = Self::index_tex_coords(obj_data);
        IndexedModel {
            positions,
            position_indices,
            tex_coords,
            tex_coord_indices,
        }
    }

    /**
     * Generated indexed position data from the given wavefront object.
     */
    fn index_positions(obj_data: &Obj) -> (Vec<f32>, Vec<u16>) {
        let mut vertex_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut positions = Vec::new();
        let position_indices = obj_data
        .vertices()
        .map(|vertex| {
            let vertex_id = vertex.position_index();
            *vertex_memo.entry(vertex_id).or_insert_with(|| {
                let index = u16::try_from(positions.len() / SIZE_POSITION).unwrap();
                positions.extend(vertex.position());
                index
            })
        })
        .collect();
        return (positions, position_indices);
    }

    /**
     * Generated indexed texture coordinate data from the given wavefront object.
     */
    fn index_tex_coords(obj_data: &Obj) -> (Vec<f32>, Vec<u16>) {
        let mut tex_coord_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        let tex_coord_indices = obj_data
            .vertices()
            .map(|vertex| {
                let tex_coord_id = vertex.uv_index().unwrap_or(0usize);
                *tex_coord_memo.entry(tex_coord_id).or_insert_with(|| {
                    let index = u16::try_from(tex_coords.len() / SIZE_TEX_COORD).unwrap();
                    let uvw = vertex.uv().unwrap_or([0.0, 0.0, 0.0]);
                    tex_coords.push(uvw[0]);
                    // Flip the V coordinate, as the Wii expects it exactly the other way around.
                    tex_coords.push(1.0 - uvw[1]);
                    index
                })
            })
            .collect();
        return (tex_coords, tex_coord_indices);
    }
}
