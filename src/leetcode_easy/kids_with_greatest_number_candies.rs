// There are n kids with candies. You are given an integer array candies, where each candies[i]
// represents the number of candies the ith kid has, and an integer extraCandies, denoting
// the number of extra candies that you have.
//
// Return a boolean array result of length n, where result[i] is true if, after giving
// the ith kid all the extraCandies, they will have the greatest number of candies among
// all the kids, or false otherwise.
//
// Note that multiple kids can have the greatest number of candies.

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result = vec![false; candies.len()];
    let max_value = candies.iter().max().unwrap();

    for i in 0..candies.len() {
        if candies[i] + extra_candies >= *max_value {
            result[i] = true;
        }
    }
    result
}

#[test]
fn test_kids_with_candies() {
    assert_eq!(
        kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    )
}
