pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![std::i32::MAX; n]; m];
        dp[0][0] = grid[0][0];
        for i in 0..m-1 {
            for j in 0..n-1 {
                dp[i+1][j] = dp[i][j].min(grid[i+1][j] + dp[i][j]);
                dp[i][j+1] = dp[i][j].min(grid[i][j+1] + dp[i][j]);
            }
        }
        dp[m-1][n-1]
    }
}