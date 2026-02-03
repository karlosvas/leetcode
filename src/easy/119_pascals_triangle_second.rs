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
