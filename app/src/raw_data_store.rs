use alloc::vec::Vec;
use alloc::collections::BTreeMap;

const ENTRY_COUNT: usize = 1;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const RAW_DATA_LIST: [(&str, &'static[u8]); ENTRY_COUNT] = [
    ("Suzanne", include_bytes!("../data/Suz.obj"))
];

/**
 * Data structure for the raw data store
 */
pub struct RawDataStore {
    raw_data_map : BTreeMap<&'static str, &'static[u8]>
}

/**
 * Implementation of the raw data store: allows for fast searching.
 */
impl RawDataStore {
    /**
     * Create a new factory.
     */
    pub fn new() -> RawDataStore {
        let mut store = RawDataStore {
            raw_data_map: BTreeMap::new()
        };
        store.load_references();
        return store;
    }

    /**
     * Load all references
     */
    fn load_references(&mut self) {
        for entry in RAW_DATA_LIST {
            self.raw_data_map.insert(entry.0, entry.1);
        }
    }

    /**
     * Return the given raw data.
     */
    pub fn get(&mut self, key: &'static str) -> Option<&&'static [u8]> {
        return self.raw_data_map.get(key);
    }

    /**
     * Return all raw models.
     */
    pub fn entries() -> Vec<(&'static str, &'static[u8])> {
        return RAW_DATA_LIST.to_vec();
    }
}