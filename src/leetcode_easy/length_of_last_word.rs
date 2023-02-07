pub fn length_of_last_word(s: String) -> i32 {
    let mut result: Vec<&str> = s.split(' ').collect();
    result.retain(|&x| x != "");
    let mut count: i32 = 0;
    if let Some(word) = result.last() {
        count = word.chars().count() as i32;
    };
    count
}

#[test]
fn test_length_of_last_word() {
    assert_eq!(length_of_last_word(String::from("Hello World")), 5);
    assert_eq!(
        length_of_last_word(String::from("   fly me   to   the moon  ")),
        4
    );
    assert_eq!(
        length_of_last_word(String::from("luffy is still joyboy")),
        6
    );
}
