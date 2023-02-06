

pub fn length_of_last_word(s: String) -> i32 {
    let mut result: Vec<&str> = s.split(' ').collect();
    result.retain(|&x| x != "");
    let mut count: i32 = 0;
    if let Some(word) = result.last() {
        count = word.chars().count() as i32;
    };
    count
}