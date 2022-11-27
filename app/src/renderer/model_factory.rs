use wavefront::{Obj};
use alloc::collections::BTreeMap;
use alloc::str::from_utf8;

const raw_model_data: [(&str, &'static[u8]); 1] = [
    ("Cartridge", include_bytes!("../../data/Cartridge.obj"))
];

pub struct ModelFactory<'a> {
    models : BTreeMap<&'a str, Obj>
}

impl<'a> ModelFactory<'a> {
    pub fn new() -> ModelFactory<'a> {
        ModelFactory {
            models: BTreeMap::new()
        }
    }

    pub fn load_models(&mut self) {
        for entry in raw_model_data {
            let key = entry.0;
            let raw_data = entry.1;
            let string_data = from_utf8(raw_data).unwrap();
            let value = Obj::from_lines(string_data.lines()).unwrap();
            self.models.insert(key, value);
        }
    }

    pub fn get_model(&mut self, key: &str) -> Option<&wavefront::Obj> {
        return self.models.get(key);
    }
}