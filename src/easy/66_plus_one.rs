impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new_digits = digits.clone();

        for i in (0..new_digits.len()).rev() {
            if new_digits[i] < 9 {
                new_digits[i] += 1;
                return new_digits;
            }
            new_digits[i] = 0;
        }

        new_digits.insert(0, 1);
        new_digits
    }
}
