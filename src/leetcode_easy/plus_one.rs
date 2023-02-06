
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut str_digits = String::new();

    for num in digits {
        str_digits += &*num.to_string();
    }
    let result = str_digits.parse::<i32>().unwrap() + 1;
    let mut result_digits: Vec<i32> = Vec::new();
    for i in result.to_string().chars() {
        result_digits.push(i.to_digit(10).unwrap() as i32);
    };
    result_digits

}


#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
}