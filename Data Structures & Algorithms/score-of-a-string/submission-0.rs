impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut res = 0;
        let s_bytes = s.as_bytes();
        let s_len = s_bytes.len();
        for i in 0..s_len-1 {
            res +=  (s_bytes[i+1] as i32 - s_bytes[i] as i32).abs();
        }
        res
    }
}
