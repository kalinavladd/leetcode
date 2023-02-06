pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for num in digits.iter_mut().rev() {
        match *num == 9 {
            true => *num = 0,
            false => {
                *num += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}


#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
    assert_eq!(plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
    assert_eq!(plus_one(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), vec![1, 2, 3, 4, 5, 6, 7, 9, 0]);
}