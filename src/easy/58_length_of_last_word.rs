impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed = s.trim_end();

        for (i, c) in trimmed.chars().rev().enumerate() {
            if c == ' ' {
                return i as i32;
            }
        }

        trimmed.len() as i32
    }
}
