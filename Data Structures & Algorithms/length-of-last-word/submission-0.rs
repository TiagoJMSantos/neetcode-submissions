impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let s_len = s_bytes.len();
        let mut res = 0;
        for i in (0..s_len).rev() {
            if s_bytes[i] != b' ' {
                res += 1;
            } else if res > 0 {
                break;
            }
        }
        res
    }
}
