pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for elem in accounts {
        let count: i32 = elem.iter().sum();
        println!("{}", count);
        if count > result {
            result = elem.iter().sum::<i32>()
        }
    }

    result
}

#[test]
fn test_maximum_wealth() {
    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
}
