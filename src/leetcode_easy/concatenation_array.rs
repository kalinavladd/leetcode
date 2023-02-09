
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut ans: Vec<i32> = nums.clone();

    ans.append(&mut nums);
    ans
}

pub fn get_concatenation2(nums: Vec<i32>) -> Vec<i32> {
    let v = nums.clone();
    [nums, v].concat()
}

pub fn get_concatenation3(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().chain(nums.iter()).cloned().collect()
}


#[test]
fn test_get_concatenation() {
    assert_eq!(get_concatenation(vec![1,2,1]), vec![1,2,1,1,2,1]);
    assert_eq!(get_concatenation2(vec![1,2,1]), vec![1,2,1,1,2,1]);
    assert_eq!(get_concatenation3(vec![1,2,1]), vec![1,2,1,1,2,1]);
}