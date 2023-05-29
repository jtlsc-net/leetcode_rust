// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// Passed, runtime 11 ms mem 2.9 mb

struct Solution;
impl Solution {

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = 1;
        let mut max_prof: i32 = 0;
        while r < prices.len() {
            if prices[r] < prices[l] {
                l = r;
            }
            else if prices[r] - prices[l] > max_prof {
                max_prof = prices[r] - prices[l];
            }
            r += 1;
        }
        return max_prof;
    }
}

fn main() {
    let prices0 = vec![7i32,6,5,4,3,2,1];
    let prices1 = vec![7i32,1,5,3,6,4];
    let prices2 = vec![7i32,6,4,3,1];
    let prices3 = vec![3i32,7,0,1,1];
    let prices4 = vec![3i32,7,5,1,6,0];
    let prices5 = vec![3i32,2,6,5,0,3];
    assert_eq!(Solution::max_profit(prices0), 0);
    println!("-");
    assert_eq!(Solution::max_profit(prices1), 5);
    println!("-");
    assert_eq!(Solution::max_profit(prices2), 0);
    println!("-");
    assert_eq!(Solution::max_profit(prices3), 4);
    println!("-");
    assert_eq!(Solution::max_profit(prices4), 5);
    println!("-");
    assert_eq!(Solution::max_profit(prices5), 4);
}