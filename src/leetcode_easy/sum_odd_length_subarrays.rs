// Given an array of positive integers arr, return the
// sum of all possible odd-length subarrays of arr.
//
// A subarray is a contiguous subsequence of the array.

pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut result = (1..=arr.len()).step_by(2).fold(0, |acc, curr_len| {
        acc + arr.windows(curr_len).flatten().sum::<i32>()
    });
    result
}

pub fn sum_odd_length_subarrays2(arr: Vec<i32>) -> i32 {
    if arr.len() == 2 {
        return arr.iter().sum::<i32>();
    }

    let mut result = arr.iter().sum::<i32>();

    if arr.len() % 2 == 0 {
        result = 0;
    }

    let mut window = vec![];

    for i in (1..arr.len()).step_by(2) {
        let mut res = arr.windows(i);
        loop {
            match res.next() {
                Some(v) => window.push(v),
                None => break,
            }
        }
    }

    result + window.into_iter().flatten().sum::<i32>()
}

#[test]
fn test_sum_odd_length_subarrays() {
    assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(sum_odd_length_subarrays2(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(sum_odd_length_subarrays2(vec![1, 2]), 3);
    assert_eq!(sum_odd_length_subarrays2(vec![10, 11, 12]), 66);
    assert_eq!(sum_odd_length_subarrays2(vec![7, 6, 8, 6]), 68);
}
