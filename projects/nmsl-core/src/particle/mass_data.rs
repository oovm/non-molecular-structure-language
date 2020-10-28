use std::collections::HashMap;

pub fn mass_data() -> HashMap<&'static str, f32> {
    let mut map = HashMap::new();
    map.insert("H", 1.0);
    return map;
}
