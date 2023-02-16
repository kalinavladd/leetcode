// Given two arrays of integers nums and index. Your task is to create
// target array under the following rules:
//
// Initially target array is empty.
// From left to right read nums[i] and index[i], insert at index index[i]
// the value nums[i] in target array.
// Repeat the previous step until there are no elements to read in nums and index.
// Return the target array.
//
// It is guaranteed that the insertion operations will be valid.

use std::iter::zip;

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for (indx, num) in zip(index, nums) {
        result.insert(indx as usize, num);
    }

    result
}

pub fn create_target_array2(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut nums = nums.iter();
    let mut index = index.iter();

    while nums.len() > 0 && index.len() > 0 {
        match (index.next(), nums.next()) {
            (Some(indx), Some(value)) => result.insert(*indx as usize, value.clone()),
            (None, None) => break,
            _ => {}
        }
    }
    result
}

#[test]
fn test_create_target_array() {
    assert_eq!(
        create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(create_target_array(vec![1], vec![0]), vec![1]);

    assert_eq!(
        create_target_array2(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        create_target_array2(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(create_target_array2(vec![1], vec![0]), vec![1]);
}
