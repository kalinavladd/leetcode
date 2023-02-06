mod leetcode_easy;


fn main() {
    let r = leetcode_easy::valid_parentheses::is_valid(String::from("{}[]("));
    println!("{}", r);
}
