// Given an integer array nums and an integer k,
// return the number of pairs (i, j) where i < j such that |nums[i] - nums[j]| == k.
//
// The value of |x| is defined as:
//
// x if x >= 0.
// -x if x < 0.

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (nums[i] - nums[j]).abs() == k {
                count += 1
            }
        }
    }
    count
}

#[test]
fn test_count_k_difference() {
    assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    assert_eq!(count_k_difference(vec![1, 3], 3), 0);
    assert_eq!(count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
}
