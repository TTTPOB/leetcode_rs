/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left_highest = 0;
        let mut right_max = vec![0; height.len()];
        for i in (0..height.len() - 1).rev() {
            println!("height right {}, max right {}", height[i+1], right_max[i+1]);
            right_max[i] = std::cmp::max(height[i + 1], right_max[i + 1])
        }
        println!("{:?}", right_max);
        for col in 1..height.len() {
            left_highest = std::cmp::max(left_highest, height[col - 1]);
            let right_highest = right_max[col];
            // let left_highest = height[0..col].iter().max().unwrap_or(&0).to_owned();
            // let right_highest = height[(col + 1)..].iter().max().unwrap_or(&0).to_owned();
            let h = height[col];
            let current_col_water = std::cmp::min(left_highest, right_highest) - h;
            if current_col_water > 0 {
                result += current_col_water
            }
        }
        result
    }
}
// @lc code=end
struct Solution;
#[test]
fn test() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
