/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

// @lc code=start
impl Solution {
    fn do_search(
        result_vec: &mut Vec<Vec<i32>>,
        current_vec: &mut Vec<i32>,
        candidates: &[i32],
        target: i32,
    ) {
        if current_vec.iter().sum::<i32>() == target {
            result_vec.push(current_vec.clone());
            return;
        }
        if current_vec.iter().sum::<i32>() > target {
            return;
        }
        for (idx, x) in candidates.iter().enumerate() {
            current_vec.push(x.to_owned());
            Solution::do_search(result_vec, current_vec, &candidates[idx..], target);
            current_vec.pop();
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result_vec = vec![];
        let mut current_vec: Vec<i32> = vec![];
        Solution::do_search(&mut result_vec, &mut current_vec, &candidates, target);
        return result_vec;
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
}
