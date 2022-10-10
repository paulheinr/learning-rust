use std::collections::HashMap;

pub fn get_median(list: &mut Vec<i32>) -> f32 {
    list.sort();
    let len = list.len();
    (match len.rem_euclid(2) {
        0 => (list[len / 2 - 1] + list[len / 2]) / 2,
        1 => list[len / 2],
        _ => 0
    }) as f32
}

pub fn get_mode(list: &mut Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for e in list {
        let count = map.entry(*e).or_insert(0);
        *count += 1;
    }
    let mut max = (0, 0);
    for (k, v) in &map {
        if *v >= max.1 {
            max = (*k, *v);
        }
    }
    max.0
}