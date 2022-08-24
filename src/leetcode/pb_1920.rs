pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    nums.iter().enumerate().for_each(|ind, &x| {
        res[ind] = nums[x as usize];
    });
    res
}