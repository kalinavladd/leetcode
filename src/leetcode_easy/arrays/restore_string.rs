// You are given a string s and an integer array indices of the same length.
// The string s will be shuffled such that the character at the ith position
// moves to indices[i] in the shuffled string.
//
// Return the shuffled string.

use std::collections::HashMap;

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut dict: HashMap<i32, char> = HashMap::new();
    let mut word = s.chars();
    let mut result = String::new();

    for i in &indices {
        dict.insert(*i, word.next().unwrap());
    }

    for i in 0..indices.len() {
        result.insert(i, *dict.get(&(i as i32)).unwrap());
    }
    result
}

// can't work with ukr or rus lang
pub fn restore_string_2(s: String, indices: Vec<i32>) -> String {
    let mut ans = vec!['_'; s.len()];
    let v = s.as_bytes();
    for i in 0..indices.len() {
        ans[indices[i] as usize] = v[i] as char;
    }
    ans.iter().collect()
}

#[test]
fn test_restore_string() {
    assert_eq!(
        restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode".to_string()
    );
}
