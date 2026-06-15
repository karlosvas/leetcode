// Problem: Longest Palindromic Substring
// Difficulty: Medium

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let old_s: Vec<char> = s.chars().collect();
        let n = old_s.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut best = (0, 0);

        for i in 0..n {
            dp[i][i] = true;
        }

        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;
                if old_s[i] == old_s[j] {
                    dp[i][j] = len == 2 || dp[i + 1][j - 1];
                    if dp[i][j] && len > (best.1 - best.0 + 1) {
                        best = (i, j);
                    }
                }
            }
        }

        s[best.0..=best.1].to_string()
    }
}

// Given a string `s`, return the longest palindromic substring in `s`.
//
// Example 1:
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
//
// Example 2:
// Input: s = "cbbd"
// Output: "bb"
//
// Constraints:
// - 1 <= s.length <= 1000
// - s consists of only digits and English letters.

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(
        c1,
        Solution::longest_palindrome(String::from("babad")),
        "bab" | "aba"
    );
    crate::check_case!(
        c6,
        Solution::longest_palindrome(String::from("racecar")),
        "racecar"
    );
    crate::check_case!(c2, Solution::longest_palindrome(String::from("cbbd")), "bb");
    crate::check_case!(c3, Solution::longest_palindrome(String::from("aa")), "aa");
    crate::check_case!(
        c4,
        Solution::longest_palindrome(String::from("ab")),
        "a" | "b"
    );
    crate::check_case!(c5, Solution::longest_palindrome(String::from("a")), "a");
}
