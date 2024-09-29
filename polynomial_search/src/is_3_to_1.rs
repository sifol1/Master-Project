use std::collections::HashMap;

pub fn is_3_to_1(vector: Vec<u32>) -> bool {
    let mut count_value_map: HashMap<u32, u32> = HashMap::new();
    for integer in vector {
        let count = count_value_map.entry(integer).or_insert(0);
        *count += 1;
        if *count > 3{
            return false;
        }
    }
    // let count = count_value(vector);
    let is_3_to_1 = count_value_map.iter().all(|(key, value)| {
        if *key == 0 {
            *value == 1 // Key 0 should appear only once
        } else {
            *value == 3 // Other keys should have values equal to 3
        }
    });

    if is_3_to_1 {
        return true;
    } else {
        return false;
    }
}
