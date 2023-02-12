// You are given an array items, where each items[i] = [typei, colori, namei]
// describes the type, color, and name of the ith item. You are also given a rule represented
// by two strings, ruleKey and ruleValue.
//
// The ith item is said to match the rule if one of the following is true:
//
// ruleKey == "type" and ruleValue == typei.
// ruleKey == "color" and ruleValue == colori.
// ruleKey == "name" and ruleValue == namei.
// Return the number of items that match the given rule.

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut result = 0;

    let index = match &rule_key as &str {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return 0,
    };

    for it in items {
        if it[index] == rule_value {
            result += 1;
        }
    }
    result
}

#[test]
fn test_count_matches() {
    assert_eq!(
        count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec![
                    "computer".to_string(),
                    "silver".to_string(),
                    "lenovo".to_string()
                ],
                vec![
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string()
                ]
            ],
            "color".to_string(),
            "silver".to_string()
        ),
        1
    );
}
