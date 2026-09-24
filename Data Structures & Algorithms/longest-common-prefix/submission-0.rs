impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let mut res = strs[0].clone();
        for i in 1..strs.len() {
            let res_bytes = res.as_bytes();
            let atual_bytes = strs[i].as_bytes();
            let limite = std::cmp::min(res_bytes.len(), atual_bytes.len());
            let mut j = 0;
            while j < limite && res_bytes[j] == atual_bytes[j] {
                j += 1;
            }
            res = res[0..j].to_string();
            if res.is_empty() {
                break;
            }
        }
        res
    }
}