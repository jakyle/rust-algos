pub fn min_operations(boxes: String) -> Vec<i32> {
    let boxes = boxes.chars().collect::<Vec<char>>();
    let mut ops = 0;
    let mut count = 0;
    let mut result: Vec<i32> = vec![0; boxes.len()];
    for i in 0..boxes.len() {
        result[i] += ops;
        count += if boxes[i] == '1' { 1 } else { 0 };
        ops += count;
    }

    ops = 0;
    count = 0;
    for i in (0..boxes.len()).rev() {
        result[i] += ops;
        count += if boxes[i] == '1' { 1 } else { 0 };
        ops += count;
    }

    result
}

#[cfg(test)]
mod min_operations_tests {
    use super::*;

    fn min_operations_test_one() {}
}
