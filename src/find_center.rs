pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut count: HashMap<i32, i32> = HashMap::new();

    for edge in edges {
        for val in edge {
            *count.entry(val).or_insert(0) += 1;
        }
    }

    if let Some(kvp) = count.iter().max_by(|a, b| a.1.cmp(&b.1)) {
        *kvp.0
    } else {
        0
    }
}
