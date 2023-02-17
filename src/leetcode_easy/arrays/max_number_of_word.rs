// A sentence is a list of words that are separated by a single space with no leading or trailing spaces.
//
// You are given an array of strings sentences, where each sentences[i] represents a single sentence.
//
// Return the maximum number of words that appear in a single sentence.

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut count: i32 = 0;
    let mut sent = Vec::new();
    for sentence in sentences.iter() {
        sent = sentence.split(" ").collect();
        if sent.len() as i32 > count {
            count = sent.len() as i32;
        }
    }
    count
}

#[test]
fn test_most_words_found() {
    assert_eq!(
        most_words_found(vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string()
        ]),
        6
    );

    assert_eq!(
        most_words_found(vec![
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string()
        ]),
        3
    );
}
