// Problem: Reverse Integer
// Difficulty: Medium

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let reverse_s = x.unsigned_abs().to_string().chars().rev().collect::<String>();
        let sol = reverse_s
            .trim_start_matches('0')
            .parse::<i32>()
            .unwrap_or_default();

        if x < 0 { -sol } else { sol }
    }
}

// Given a signed 32-bit integer `x`, return `x` _with its digits reversed_. If reversing `x` causes the value to go outside the signed 32-bit integer range `[-2^31, 2^31 - 1]`, then return `0`.
//
// **Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**
//
// Example 1:
// Input: x = 123
// Output: 321
//
// Example 2:
// Input: x = -123
// Output: -321
//
// Example 3:
// Input: x = 120
// Output: 21
//
// Constraints:
// - `-2^31 <= x <= 2^31 - 1`

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(c1, Solution::reverse(123), 321);
    crate::check_case!(c2, Solution::reverse(-123), -321);
    crate::check_case!(c3, Solution::reverse(120), 21);
    crate::check_case!(c4, Solution::reverse(1534236469), 0);
    crate::check_case!(c5, Solution::reverse(-2147483648), 0);
}

