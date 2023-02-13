// You are given a positive integer array nums.
//
// The element sum is the sum of all the elements in nums.
// The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.
// Return the absolute difference between the element sum and digit sum of nums.
//
// Note that the absolute difference between two integers x and y is defined as |x - y|.

pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let element_sum = nums.iter().sum::<i32>();

    let mut digit_sum = 0;

    let binding = nums
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    for i in binding.chars() {
        digit_sum += i.to_digit(10).unwrap() as i32;
    }

    result = element_sum - digit_sum;
    result
}

fn difference_of_sum_2(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut digits = 0;

        for mut a in nums {
            while a > 0 {
                digits += a % 10;
                println!("{}", digits);
                a /= 10;
            }
        }

        i32::abs(sum - digits)
    }


#[test]
fn test_difference_of_sum() {
    assert_eq!(difference_of_sum_2(vec![1, 15, 6, 3]), 9);
    assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
}
