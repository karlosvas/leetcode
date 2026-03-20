use crate::easy::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut map_visited: HashMap<i32, i32> = HashMap::new();
        let mut aux: i32 = 0;
        let mut chars_list: String = n.to_string().clone();

        while aux != 1 {
            aux = chars_list
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(2) as i32)
                .sum();

            *map_visited.entry(aux).or_insert(0) += 1;
            if map_visited[&aux] > 1 {
                return false;
            }

            chars_list = aux.to_string();
        }

        true
    }
}

// Write an algorithm to determine if a number n is happy.

// A happy number is a number defined by the following process:

// Starting with any positive integer, replace the number by the sum of the squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
// Return true if n is a happy number, and false if not.

// Example 1:

// Input: n = 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1
// Example 2:

// Input: n = 2
// Output: false

// Constraints:

// 1 <= n <= 231 - 1
