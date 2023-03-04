/*
 * @lc app=leetcode id=40 lang=rust
 *
 * [40] Combination Sum II
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
            if idx == 0 || (*x != candidates[idx - 1]) {
                Solution::do_search(result_vec, current_vec, &candidates[(idx + 1)..], target);
            }
            current_vec.pop();
        }
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates.clone();
        candidates.sort();
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
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}
