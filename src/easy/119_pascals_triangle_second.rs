use crate::easy::Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut sol: Vec<Vec<i32>> = Vec::new();
        let mut i: i32 = 0;
        loop {
            let mut aux: Vec<i32> = Vec::new();
            for j in 0..i + 1 {
                let mut sum: i32 = 0;
                if i == 0 || j == 0 || j == i {
                    sum = 1;
                } else {
                    sum += sol[i as usize - 1][j as usize];
                    sum += sol[i as usize - 1][j as usize - 1];
                }
                aux.push(sum);
            }
            if row_index == i {
                return aux;
            }
            sol.push(aux);
            i += 1;
        }
    }
}

// 119. Pascal's Triangle II
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:


 

// Example 1:

// Input: rowIndex = 3
// Output: [1,3,3,1]
// Example 2:

// Input: rowIndex = 0
// Output: [1]
// Example 3:

// Input: rowIndex = 1
// Output: [1,1]
 

// Constraints:

// 0 <= rowIndex <= 33
