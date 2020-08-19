pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut counter: i32 = 0;
        let  s = s.into_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for j in 0..s.len() {
            for i in 0..j+1 {
                if i == j{
                    dp[i][j] = true;
                    counter += 1;
                }

                else if j - i == 1  && s[i] == s[j] {
                    dp[i][j] = true;
                    counter += 1;
                }

                else if j - i > 1 && s[i] == s[j] && dp[i+1][j-1] == true  {
                    dp[i][j] = true;
                    counter += 1;
                } 
            }
        }
        return counter;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }
}