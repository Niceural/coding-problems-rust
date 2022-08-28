// with build in functions
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut k = k as usize % nums.len();
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

// with recursion
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut k = k as usize % nums.len();
    Solution::swap(nums, k, 0);
}
fn swap(nums: &mut Vec<i32>, k: usize, id: usize) {
    if (id >= nums.len()) {
        return;
    }
    
    let val = nums[id];
    let id_pair = (id + k) % nums.len();
    Solution::swap(nums, k, id+1);
    nums[id_pair] = val;
}