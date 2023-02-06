use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let mut result: Vec<i32> = vec![];
    let mut seen = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        if seen.contains_key(&(target - num)) {
            result.push(seen[&(target - num)] as i32);
            result.push(index as i32);
            println!("{:?}", result);
            return result;
        }
        seen.insert(*num, index);
    };
    result
}
// let r = two_sum(vec![3,2,4], 6);