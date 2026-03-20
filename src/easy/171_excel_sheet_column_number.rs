use crate::easy::Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let alphabet: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut sol: i32 = 0;

        for (i, letter) in column_title.chars().rev().enumerate() {
            let index: usize = alphabet.chars().position(|c| c == letter).unwrap();
            sol += (index as i32 + 1) * (26_i32.pow(i as u32)) as i32;
        }

        sol
    }
}

// 171. Excel Sheet Column Number
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.

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
// Input: columnTitle = "A"
// Output: 1
// Example 2:

// Input: columnTitle = "AB"
// Output: 28
// Example 3:

// Input: columnTitle = "ZY"
// Output: 701

// Constraints:

// 1 <= columnTitle.length <= 7
// columnTitle consists only of uppercase English letters.
// columnTitle is in the range ["A", "FXSHRXW"].
