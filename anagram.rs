// https://leetcode.com/problems/valid-anagram/
// Passed runtime: 0 ms memory: 2.7 mb

struct Solution;
// Paste solution impl here
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_vec : Vec<char> = s.chars().collect::<Vec<_>>();
        let mut t_vec : Vec<char> = t.chars().collect::<Vec<_>>();
        s_vec.sort_unstable();
        t_vec.sort_unstable();
        return s_vec == t_vec;
    }
}

fn main(){
    // let vec1 = vec![];
    let s0 = String::from("anagram");
    let t0 = String::from("nagaram");
    let s1 = String::from("rat");
    let t1 = String::from("car");
    // println!("{}", Solution::<>(vec1));
    assert_eq!(Solution::is_anagram(s0, t0), true);
    assert_eq!(Solution::is_anagram(s1, t1), false);
}