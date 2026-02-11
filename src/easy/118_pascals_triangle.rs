use crate::easy::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut sol: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows {
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
            sol.push(aux);
        }

        sol
    }
}

// 118. Pascal's Triangle
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Given an integer numRows, return the first numRows of Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:


 

// Example 1:

// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:

// Input: numRows = 1
// Output: [[1]]
 

// Constraints:

// 1 <= numRows <= 30