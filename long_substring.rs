// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Passed
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let char_vect : Vec<char> = s.chars().collect::<Vec<_>>();
        let mut max_size = 0;
        let mut l = 0;
        let mut char_set = HashSet::new();
        for r in 0..char_vect.len() {
            while char_set.contains(&char_vect[r]){
                char_set.remove(&char_vect[l]);
                l += 1;
            }
            char_set.insert(&char_vect[r]);
            max_size = max_size.max((r-l + 1) as i32);
        }
        return max_size;
    }
}

fn main(){
    // let vec1 = vec![];
    let s0 = String::from("abcabcbb");
    let s1 = String::from("bbbbb");
    let s2 = String::from("pwwkew");
    let s3 = String::from("abcaq");
    let s4 = String::from("abba");
    assert_eq!(Solution::length_of_longest_substring(s0), 3);
    println!("-");
    assert_eq!(Solution::length_of_longest_substring(s1), 1);
    println!("-");
    assert_eq!(Solution::length_of_longest_substring(s2), 3);
    println!("-");
    assert_eq!(Solution::length_of_longest_substring(s3), 4);
    println!("-");
    assert_eq!(Solution::length_of_longest_substring(s4), 2);
}