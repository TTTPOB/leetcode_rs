/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut max = 0;
        for i in 0..prices.len() {
            min = std::cmp::min(min, prices[i]);
            max = std::cmp::max(prices[i] - min, max);
        }
        max
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}