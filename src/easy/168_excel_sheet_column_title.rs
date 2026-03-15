use crate::easy::Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let alphabet: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut sol: Vec<char> = Vec::new();
        let mut i = column_number.clone();

        while i > 0 {
            i -= 1;
            sol.push(alphabet.chars().nth((i % 26) as usize).unwrap());
            i /= 26;
        }

        sol.iter().rev().collect()
    }
}

// 168. Excel Sheet Column Title
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.

// For example:
// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28
// ...

// Example 1:

// Input: columnNumber = 1
// Output: "A"
// Example 2:

// Input: columnNumber = 28
// Output: "AB"
// Example 3:

// Input: columnNumber = 701
// Output: "ZY"

// Constraints:

// 1 <= columnNumber <= 231 - 1
