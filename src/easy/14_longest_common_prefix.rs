impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut min_len: usize = usize::MAX;
        let mut min_str: String = String::new();
        for i in &strs {
            if i.len() < min_len {
                min_len = i.len();
                min_str = i.to_string();
            }
        }

        let mut index = 0;
        for &char_min_str in min_str.as_bytes() {
            for s in &strs {
                if s.as_bytes()[index] != char_min_str {
                    return strs[0][0..index].to_string();
                }
            }
            index += 1;
        }

        strs[0][0..index].to_string()
    }
}
