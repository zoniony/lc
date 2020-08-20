fn main() {
    let a = vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]];
    let b = Solution::min_path_sum(a);
    println!("{}",b)
}

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![std::i32::MAX; n]; m];
        dp[0][0] = grid[0][0];
        for i in 0..m {
            for j in 0..n {
                if i == m - 1 && j == n - 1{
                    break;
                }
                else if j == n - 1 {
                    dp[i+1][j] = dp[i+1][j].min(grid[i+1][j] + dp[i][j]);
                }
                else if i == m - 1 {
                    dp[i][j+1] = dp[i][j+1].min(grid[i][j+1] + dp[i][j]);
                }
                else {
                    dp[i+1][j] = dp[i+1][j].min(grid[i+1][j] + dp[i][j]);
                    dp[i][j+1] = dp[i][j+1].min(grid[i][j+1] + dp[i][j]);
                }
            }
        }
        dp[m-1][n-1]
    }
}