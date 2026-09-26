impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let arlen = arr.len();
        let mut tempI = 0;
        let mut currentMax = 0;
        for i in (0..arlen).rev() {
            tempI = arr[i];
            arr[i] = currentMax;
            if tempI > currentMax {
                currentMax = tempI;
            }
        }
        arr[arlen-1] = -1;
        arr
    }
}
