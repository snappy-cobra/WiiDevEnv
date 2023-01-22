use grrustlib::{GX_BeginDispList, GX_EndDispList, DCInvalidateRange};
use libc::{c_void, memalign, free, realloc};

const ALIGN_SIZE: usize = 32;
const DEFAULT_LIST_SIZE: usize = 1024;

/**
 * Caches display lists based on the textured model name.
 */
pub struct DisplayCache {
    display_list_map: BTreeMap<TexturedModelName, c_void>,
}

impl DisplayCache {
    pub fn get_display_list(&self, key: &TexturedModelName) -> DisplayList {
        match self.display_list_map.get(key) {
            Some(display_list) => display_list,
            None => {
                let display_list =  DisplayList::new();
                self.display_list_map.insert(key, display_list);
                display_list
            } 
        }
    }
}

/**
 * Interface for creating a GX Display list.
 */
pub struct DisplayList {
    initialized: bool,
    list_size: usize,
    gx_list: *mut c_void
}

impl DisplayList {
    pub fn new() -> Self {
        let list_size = ALIGN_SIZE * DEFAULT_LIST_SIZE;
        unsafe {
            Self {
                initialized: false,
                list_size,
                gx_list: memalign(ALIGN_SIZE, list_size)
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn open(&mut self) {
        unsafe {
            DCInvalidateRange(self.gx_list, self.list_size);
            GX_BeginDispList(self.gx_list, self.list_size);
        }
        self.initialized = false;
    }

    pub fn close(&mut self) {
        unsafe {
            self.list_size = GX_EndDispList();
            realloc(self.gx_list, self.list_size);
        }
        self.initialized = true;
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
