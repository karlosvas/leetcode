impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(indice) = haystack.find(&needle) {
            return indice as i32;
        } else {
            return -1;
        }
    }
}
