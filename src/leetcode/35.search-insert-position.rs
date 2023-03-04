/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                l = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                r = mid - 1;
            }
        }
        l as i32
    }
}
// @lc code=end
// this has O(n) time complexity
// pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//     for (idx, n) in nums.iter().enumerate() {
//         if *n >= target {
//             return idx as i32;
//         }
//     }
//     nums.len() as i32
// }
struct Solution;
#[test]
fn test() {
    // assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}
