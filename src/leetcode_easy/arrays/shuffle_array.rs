use std::iter::zip;

// pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
//     let mut result: Vec<i32> = vec![];
//
//     for (elem1, elem2) in zip(
//         &nums[0..n.clone() as usize].to_vec(),
//         &nums[n as usize..nums.len()].to_vec()) {
//
//         result.push(*elem1);
//         result.push(*elem2);
//     }
//     result
// }

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result: Vec<i32> = vec![];

    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[i + n]);
    }
    result
}

#[test]
fn test_shuffle() {
    assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    assert_eq!(
        shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
        vec![1, 4, 2, 3, 3, 2, 4, 1]
    );
}
