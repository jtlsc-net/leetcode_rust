// https://leetcode.com/problems/valid-parentheses/description/
// passed runtime 1ms memory 2.1 mb

struct Solution;
// Paste solution impl here
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for x in s.chars() {
            if x == '(' || x == '[' || x == '{' {
                stack.push(x);
            }
            else if x == ')' && stack.pop() != Some('(') {
                return false;
            }
            else if x == ']' && stack.pop() != Some('[') {
                return false;
            }
            else if x == '}' && stack.pop() != Some('{') {
                return false;
            }
        }
        if stack.len() != 0 {
            return false;
        }
        return true;
    }
}

fn main(){
    // let vec1 = vec![];
    // println!("{}", Solution::<>(vec1));
    let s0 = String::from("()");
    let s1 = String::from("()[]{}");
    let s2 = String::from("(]");
    assert_eq!(Solution::is_valid(s0), true);
    assert_eq!(Solution::is_valid(s1), true);
    assert_eq!(Solution::is_valid(s2), false);
}