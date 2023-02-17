// A sentence is a list of words that are separated by a single space with no leading or trailing
// spaces. Each of the words consists of only uppercase and lowercase
// English letters (no punctuation).
//
// For example, "Hello World", "HELLO", and "hello world hello world" are all sentences.
// You are given a sentence s and an integer k. You want to truncate s
// such that it contains only the first k words. Return s after truncating it.

// 0ms
pub fn truncate_sentence(s: String, k: i32) -> String {
    s.split_whitespace().collect::<Vec<_>>()[0..k as usize].join(" ")
}

#[test]
fn test_truncate_sentence() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you".to_string()
    );
}
