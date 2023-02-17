pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();

    for elem in 1..nums.len() {
        result[elem] += result[elem - 1]
    }

    result
}

#[test]
fn test_running_sum() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
}
