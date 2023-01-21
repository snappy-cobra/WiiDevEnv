use libc::c_void;

pub struct DisplayCache {
    display_list_map: BTreeMap<TexturedModelName, c_void>,
}

pub impl DisplayCache {
    pub fn set_display_list(&self, key: &TexturedModelName) -> Option<c_void> {
        self.display_list_map.get(key)
    }
    pub 
}