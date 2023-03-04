/*
 * @lc app=leetcode id=41 lang=rust
 *
 * [41] First Missing Positive
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 0;
        // this loop is to make sure all the positive number is in the right place
        // 1 at index 0, 2 at index 1, ...
        while i < nums.len() {
            let v = nums[i];
            // negative number
            if v > 0
                // already in the right place
                && v != (i as i32 + 1)
                // exceed the nums array
                && (v - 1) < nums.len() as i32
                // swap will do nothing and cause infinite loop
                && nums[v as usize - 1] != v
            {
                nums.swap(i, v as usize - 1)
            } else {
                i += 1;
            }
        }
        // after all number has been in the right place, the number at index i
        // should be i + 1, otherwise, the positive number array will not be consecutive
        // so the i + 1 should be the first missing number
        for (idx, n) in nums.iter().enumerate() {
            if idx as i32 != n - 1 {
                return (idx + 1) as i32;
            }
        }
        // if all positive numbers in the array is consecutive, and from 1
        // the smallest missing number is the length of the array + 1
        nums.len() as i32 + 1
    }
}
// @lc code=end
struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5]), 6);
}
