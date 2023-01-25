use grrustlib::{GX_BeginDispList, GX_EndDispList, DCInvalidateRange, GX_CallDispList};
use libc::{c_void, memalign, free, realloc};
use alloc::collections::BTreeMap;
use super::textured_model::TexturedModelName;

const ALIGN_SIZE: u32 = 32;
const DEFAULT_LIST_SIZE: u32 = 1024;

/**
 * Caches display lists based on the textured model name.
 */
pub struct DisplayCache {
    display_list_map: BTreeMap<TexturedModelName, DisplayList>,
}

impl DisplayCache {
    pub fn new() -> Self {
        return Self {
            display_list_map : Default::default()
        }
    }

    pub fn get_display_list(&mut self, key: &TexturedModelName) -> &mut DisplayList {
        if !self.display_list_map.contains_key(key) {
            self.display_list_map.insert(key.clone(), DisplayList::new());
        }
        return self.display_list_map.get_mut(key).unwrap();
    }
}

/**
 * Interface for creating a GX Display list.
 */
pub struct DisplayList {
    initialized: bool,
    list_size: u32,
    gx_list: *mut c_void
}

impl DisplayList {
    pub fn new() -> Self {
        let list_size = ALIGN_SIZE * DEFAULT_LIST_SIZE;
        unsafe {
            Self {
                initialized: false,
                list_size,
                gx_list: memalign(ALIGN_SIZE as usize, list_size as usize)
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn open(&mut self) {
        // If created before, resize to the default size.
        if self.initialized {
            self.list_size = ALIGN_SIZE * DEFAULT_LIST_SIZE;
            unsafe {
                realloc(self.gx_list, self.list_size as usize);
            }
        }

        // Invalidate the cache and initialize
        unsafe {
            DCInvalidateRange(self.gx_list, self.list_size);
            GX_BeginDispList(self.gx_list, self.list_size);
        }
        self.initialized = false;
    }

    pub fn close(&mut self) {
        // Close the list and adjust the size
        unsafe {
            self.list_size = GX_EndDispList();
            realloc(self.gx_list, self.list_size as usize);
        }
        self.initialized = true;
    }

    pub fn set_active(&mut self) {
        // Tell the wii the display list is to be used, if it is initialized.
        if self.is_initialized() {
            unsafe {
                GX_CallDispList(self.gx_list, self.list_size);
            }
        }
    }
}

impl Drop for DisplayList {
    /// Cleanup the display list
    fn drop(&mut self) {
        unsafe {
            free(self.gx_list);
        }
    }
}
