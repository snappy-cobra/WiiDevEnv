use crate::renderer::{GRRLIB_texImg, GRRLIB_LoadTexture};

/**
 * Our representation of a texture.
 */
pub struct Texture {
    grrlib_texture: *mut GRRLIB_texImg
}

/**
 * Implementation of texture.
 */
impl Texture {
    pub fn new(png_data: &mut Vec<u8>) -> Texture {
        unsafe {
            let grrlib_texture = GRRLIB_LoadTexture(png_data.as_ptr());
            return Texture { grrlib_texture };
        }
    }
}
