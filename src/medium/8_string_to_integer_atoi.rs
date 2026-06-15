// Problem: String to Integer (atoi)
// Difficulty: Medium

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        use std::num::IntErrorKind;
        let trimmed = s.trim_start();
        let pos = trimmed
            .bytes()
            .enumerate()
            .position(|(i, b)| !(b.is_ascii_digit() || (i == 0 && matches!(b, b'+' | b'-'))))
            .unwrap_or(trimmed.len());

        match trimmed[..pos].parse::<i64>() {
            Ok(n) => n.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => i32::MAX,
                IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}

// Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.
//
// The algorithm for `myAtoi(string s)` is as follows:
//
// 1. **Whitespace**: Ignore any leading whitespace (`" "`).
// 2. **Signedness**: Determine the sign by checking if the next character is `'-'` or `'+'`, assuming positivity if neither present.
// 3. **Conversion**: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
// 4. **Rounding**: If the integer is out of the 32-bit signed integer range `[-2^31, 2^31 - 1]`, then round the integer to remain in the range. Specifically, integers less than `-2^31` should be rounded to `-2^31`, and integers greater than `2^31 - 1` should be rounded to `2^31 - 1`.
//
// Return the integer as the final result.
//
// Example 1:
// Input: s = "42"
// Output: 42
//
// Example 2:
// Input: s = " -042"
// Output: -42
//
// Example 3:
// Input: s = "1337c0d3"
// Output: 1337
//
// Example 4:
// Input: s = "0-1"
// Output: 0
//
// Example 5:
// Input: s = "words and 987"
// Output: 0
//
// Constraints:
// - `0 <= s.length <= 200`
// - `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`.

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(c1, Solution::my_atoi("42".to_string()), 42);
    crate::check_case!(c2, Solution::my_atoi(" -042".to_string()), -42);
    crate::check_case!(c3, Solution::my_atoi("1337c0d3".to_string()), 1337);
    crate::check_case!(c4, Solution::my_atoi("0-1".to_string()), 0);
    crate::check_case!(c5, Solution::my_atoi("words and 987".to_string()), 0);
    crate::check_case!(
        c6,
        Solution::my_atoi("-91283472332".to_string()),
        -2147483648
    );
    crate::check_case!(c7, Solution::my_atoi("91283472332".to_string()), 2147483647);
    crate::check_case!(c8, Solution::my_atoi("".to_string()), 0);
    crate::check_case!(c9, Solution::my_atoi("   +0 123".to_string()), 0);
    crate::check_case!(c10, Solution::my_atoi("+1".to_string()), 1);
}
