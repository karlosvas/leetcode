// Problem: Zigzag Conversion
// Difficulty: Medium

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let period = 2 * (num_rows - 1);
        let mut filas: Vec<String> = vec![String::new(); num_rows];

        for (i, &c) in chars.iter().enumerate() {
            let r = i % period;
            let fila = if r < num_rows { r } else { period - r };
            filas[fila].push(c);
        }

        filas.concat()
    }
}

// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
// (you may want to display this pattern in a fixed font for better legibility)
//
// P   A   H   N
// A P L S I I G
// Y   I   R
//
// And then read line by line: "PAHNAPLSIIGYIR"
//
// Write the code that will take a string and make this conversion given a number of rows:
//
// string convert(string s, int numRows);
//
// Example 1:
// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"
//
// Example 2:
//
// P   I   N
// A A S R G
// Y L H I
// P   I
//
// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"
//
// Example 3:
// Input: s = "A", numRows = 1
// Output: "A"
//
// Constraints:
// 1 <= s.length <= 1000
// s consists of English letters (lower-case and upper-case), ',' and '.'.
// 1 <= numRows <= 1000

#[cfg(test)]
mod tests {
    use super::*;

    crate::check_case!(
        c1,
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR"
    );
    crate::check_case!(
        c2,
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI"
    );
    crate::check_case!(c3, Solution::convert("A".to_string(), 1), "A");
}
