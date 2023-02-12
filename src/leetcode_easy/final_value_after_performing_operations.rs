pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut result = 0;
    for operation in operations {
        match &operation as &str {
            "X++" | "++X" => result += 1,
            "--X" | "X--" => result -= 1,
            _ => {}
        }
    }
    result
}

#[test]
fn test_final_value_after_operations() {
    assert_eq!(
        final_value_after_operations(vec![
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string()
        ]),
        1
    );
    assert_eq!(
        final_value_after_operations(vec![
            "++X".to_string(),
            "++X".to_string(),
            "X++".to_string()
        ]),
        3
    );
}
