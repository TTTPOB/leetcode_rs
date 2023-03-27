/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        } else if nums.len() <= 2 {
            return nums.iter().max().unwrap().clone();
        }
        let mut result = vec![0; nums.len()];
        result[0] = nums[0];
        result[1] = nums[1];
        result[2] = nums[0] + nums[2];
        for i in 3..(result.len()) {
            result[i] = std::cmp::max(result[i - 2], result[i - 3]) + nums[i];
        }
        return result[(result.len() - 2)..].iter().max().unwrap().clone();
    }
}
// @lc code=end
struct Solution;
#[test]
fn test() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
