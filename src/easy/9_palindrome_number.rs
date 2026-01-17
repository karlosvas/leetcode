impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let new_x: Vec<char> = x.to_string().chars().collect();

        for index in 0..(new_x.len() / 2) {
            if new_x[index] != new_x[new_x.len() - 1 - index] {
                return false;
            }
        }

        true
    }
}
