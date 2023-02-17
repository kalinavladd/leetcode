// We are given a list nums of integers representing a list compressed with run-length encoding.
//
// Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).
// For each such pair, there are freq elements with value val concatenated in a sublist.
// Concatenate all the sublists from left to right to generate the decompressed list.
//
// Return the decompressed list.

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut freq_index = 0;
    let mut val_index = 1;

    while val_index < nums.len() {
        for i in 0..nums[freq_index] {
            result.push(nums[val_index]);
        }
        freq_index += 2;
        val_index += 2;
    }

    return result;
}

#[test]
fn test_decompress_rl_elist() {
    assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}
