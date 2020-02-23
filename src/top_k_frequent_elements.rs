

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

    use std::collections::HashMap;
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut number_frequency: BinaryHeap<Reverse<(i32, i32)>> = nums.iter()
        .fold(HashMap::new(), |mut acc, i| {
            acc.entry(i)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            acc
        })
        .iter()
        .fold(BinaryHeap::new(), |mut min_heap, (num, freq)| {

            if min_heap.len() < k as usize {
                min_heap.push(Reverse((*freq, **num)));
            } else {
                let count = min_heap.peek().unwrap().0 .0;
                if *freq > count {
                    min_heap.pop();
                    min_heap.push(Reverse((*freq, **num)))
                }
            }

            min_heap
        });

        let mut res = vec![];
        while !number_frequency.is_empty() {
            res.insert(0, number_frequency.pop().unwrap().0 .1);
        }
        res
}



#[cfg(test)]
mod top_k_frequent {
    use super::*;

    #[test]
    fn top_2_frequent() {
        // arrange
        let test = vec![1,1,1,2,2,3];
        let k = 2;

        // act
        let result = top_k_frequent(test, k);

        // assert
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn one_entry() {
        // arrange
        let test = vec![1];
        let k = 1;

        // act
        let result = top_k_frequent(test, k);

        // assert
        assert_eq!(result, vec![1]);
    }

}