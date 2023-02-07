//You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.
//
// Letters are case sensitive, so "a" is considered a different type of stone from "A".

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count = 0;
    for i in stones.chars() {
        if jewels.contains(*&i) {
            count += 1
        };
    }
    count
}

#[test]
fn test_num_jewels_in_stones() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
}
