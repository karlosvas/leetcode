impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut i: i64 = 1;
        while i * i <= x as i64 {
            i += 1;
        }

        (i - 1) as i32
    }
}
