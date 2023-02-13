pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for i in 0..nums.len() {
        let main_value = nums[i];
        let mut count = 0;

        for val in &nums {
            if val < &main_value {
                count += 1
            }
        }
        result.push(count)
    }

    result
}

#[test]
fn test_smaller_numbers_than_current() {
    assert_eq!(
        smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
}
