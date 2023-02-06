mod leetcode_easy;


fn main() {
    let r = leetcode_easy::valid_parentheses::is_valid(String::from("{}[]("));
    println!("{}", r);

    leetcode_easy::plus_one::plus_one(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // println!("{:?}", plus);
}
