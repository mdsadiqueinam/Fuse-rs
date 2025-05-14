use std::collections::HashMap;

pub struct KeyStore {
    keys: Vec<String>,
    
    key_map: HashMap<String, usize>,
}