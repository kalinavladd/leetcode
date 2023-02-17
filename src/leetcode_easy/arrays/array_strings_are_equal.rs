// Given two string arrays word1 and word2, return true if the two
// arrays represent the same string, and false otherwise.
//
// A string is represented by an array if the array elements concatenated in order forms the string.

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    return if word1
        .iter()
        .map(|x| x.chars())
        .flatten()
        .collect::<String>()
        == word2
            .iter()
            .map(|x| x.chars())
            .flatten()
            .collect::<String>()
    {
        true
    } else {
        false
    };
}

pub fn array_strings_are_equal2(word1: Vec<String>, word2: Vec<String>) -> bool {
    let first_str: String = word1.into_iter().collect();
    let second_str: String = word2.into_iter().collect();

    first_str == second_str
}

#[test]
fn test_array_strings_are_equal() {
    assert_eq!(
        array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ),
        true
    );
    assert_eq!(
        array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ),
        false
    );
    assert_eq!(
        array_strings_are_equal2(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ),
        true
    );
    assert_eq!(
        array_strings_are_equal2(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ),
        false
    );
}
