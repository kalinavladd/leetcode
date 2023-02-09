// Given a zero-based permutation nums (0-indexed), build an array ans of the same length
// where ans[i] = nums[nums[i]] for each 0 <= i < nums.length and return it.
//
// A zero-based permutation nums is an array of distinct integers from 0 to nums.length - 1 (inclusive).



pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; nums.len()];

    for i in 0..nums.len() {
        result[i] = nums[nums[i] as usize]
    }
    result
}



fn test_build_array() {
    assert_eq!(build_array(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3]);
    assert_eq!(build_array(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
}