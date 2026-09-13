impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        if nums.len() < 1 || nums.len() > 1000 { return res };
        for i in 1..=2 {
            for i in 0..nums.len() {
                res.push(nums[i]);
            }
        }
        return res;
    }
}
