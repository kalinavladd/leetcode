// Given an array of integers nums, return the number of good pairs.
//
// A pair (i, j) is called good if nums[i] == nums[j] and i < j.

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for i in 0..nums.len() {
        for j in 1..nums.len() {
            if nums[i] == nums[j] && j > i {
                result += 1
            }
        }
    }
    result
}



#[test]
fn test_num_identical_pairs() {
    assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
}
