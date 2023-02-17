// You are given a string allowed consisting of distinct
// characters and an array of strings words. A string is consistent
// if all characters in the string appear in the string allowed.
//
// Return the number of consistent strings in the array words.

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut result = 0;

    for word in words.iter() {
        for char in word.chars() {
            if !allowed.chars().any(|x| x == char) {
                result += 1;
                break;
            }
        }
    }
    words.len() as i32 - result
}

#[test]
fn test_count_consistent_strings() {
    assert_eq!(
        count_consistent_strings(
            "ab".to_string(),
            vec![
                "ad".to_string(),
                "bd".to_string(),
                "aaab".to_string(),
                "baa".to_string(),
                "badab".to_string()
            ]
        ),
        2
    );
}
