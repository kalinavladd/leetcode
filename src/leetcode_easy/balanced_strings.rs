
pub fn balanced_string_split(s: String) -> i32 {

    let mut stack = 0;
    let mut result = 0;

    for char in s.chars() {
        match char {
            'R' => stack += 1,
            'L' => stack -= 1,
            _ => panic!()
        }
        if stack == 0 {
            result += 1
        }
    }
    result
}


#[test]
fn test_balanced_string_split() {
    assert_eq!(balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(balanced_string_split("RLRRRLLRLL".to_string()), 2);
    assert_eq!(balanced_string_split("LLLLRRRR".to_string()), 1);
}