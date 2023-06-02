// https://leetcode.com/problems/valid-palindrome/description/
// Passed runtime 0ms, memory 2.8mb
struct Solution;
// Paste solution impl here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut letters = Vec::new();
        for x in s.to_lowercase().chars() {
            if x.is_alphanumeric() {
                letters.push(x);
            }
        }
        let mut l = 0;
        if letters.len() == 0 {
            return true;
        }
        let mut r = letters.len() - 1;
        while l < r {
            if letters[l] != letters[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        return true;
    }
}

fn main(){
    // let vec1 = vec![];
    // println!("{}", Solution::<>(vec1));
    let s0 = String::from("A man, a plan, a canal: Panama");
    let s1 = String::from("race a car");
    let s2 = String::from("");
    assert_eq!(Solution::is_palindrome(s0), true);
    assert_eq!(Solution::is_palindrome(s1), false);
    assert_eq!(Solution::is_palindrome(s2), true);
}