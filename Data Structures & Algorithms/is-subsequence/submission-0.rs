impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let s_len = s_bytes.len();
        if s_len == 0 {return true;}
        let t_len = t_bytes.len();
        if s_len > t_len {
            return false;
        }
        let mut t_ptr = 0;
        let mut s_ptr = 0;
        while (s_ptr < s_len) && (t_ptr < t_len) {
            if s_bytes[s_ptr] == t_bytes[t_ptr] {s_ptr +=1;}
            t_ptr += 1;
        }
        s_ptr == s_len
    }
}
