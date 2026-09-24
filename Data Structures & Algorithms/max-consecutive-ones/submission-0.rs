impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res : i32 = 0;
        let mut temp_res : i32 = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                temp_res += 1;
            } else {
                if res < temp_res {
                    res = temp_res;
                }
                temp_res = 0; 
            }
        }
        if res < temp_res {
            res = temp_res;
        }
        res
    }
}
