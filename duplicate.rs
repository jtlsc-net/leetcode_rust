// https://leetcode.com/problems/contains-duplicate/description/
// Passed, runtime 19 ms, 3.1 mb memory
struct Solution;
impl Solution {
    pub fn contains_duplicate(&self, nums: Vec<i32>) -> bool {
        let mut found_nums = nums;
        found_nums.sort_unstable();
        let mut last = -1;
        let mut first_pass = false;
        let mut match_found = false;
        for x in found_nums.iter(){
            if first_pass {
                first_pass = true;
                last = *x;
            }
            else if last == *x {
                match_found = true;
                return match_found;
            }
            else {
                last = *x;
            }
        }
        return match_found;
    }
}

fn main() {
    let vec1 = vec![1i32, 2, 3, 1];
    let vec2 = vec![1i32, 2, 3, 4];
    let vec3 = vec![1i32,1,1,3,3,4,3,2,4,2];
    let sol = Solution;
    println!("{}", sol.contains_duplicate(vec1));
    println!("{}", sol.contains_duplicate(vec2));
    println!("{}", sol.contains_duplicate(vec3));
}