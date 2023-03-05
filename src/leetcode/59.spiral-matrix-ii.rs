/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mtx = vec![vec![0; n]; n];
        let mut radius = n;
        let mut i = 0;
        let mut j = 0;
        for num in 1..=(n * n) {
            let upper_edge_row_idx = (n - radius) / 2;
            let bottom_edge_row_idx = upper_edge_row_idx + radius - 1;
            let left_edge_col_idx = upper_edge_row_idx;
            let right_edge_col_idx = bottom_edge_row_idx;
            mtx[i][j] = num as i32;
            if i == upper_edge_row_idx && j < right_edge_col_idx {
                j += 1;
            } else if j == right_edge_col_idx && i < bottom_edge_row_idx {
                i += 1;
            } else if i == bottom_edge_row_idx && j > left_edge_col_idx {
                j -= 1;
            } else if j == left_edge_col_idx && i > upper_edge_row_idx {
                if i == upper_edge_row_idx + 1 {
                    radius -= 2;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
        mtx
    }
}
// @lc code=end
struct Solution;

#[test]
fn test() {
    // assert_eq!(
    //     Solution::generate_matrix(3),
    //     vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    // );
    assert_eq!(
        Solution::generate_matrix(6),
        vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![20, 21, 22, 23, 24, 7],
            vec![19, 32, 33, 34, 25, 8],
            vec![18, 31, 36, 35, 26, 9],
            vec![17, 30, 29, 28, 27, 10],
            vec![16, 15, 14, 13, 12, 11]
        ]
    );
}
