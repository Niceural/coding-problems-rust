pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    // fastest solution
    let mut res = vec![0; nums.len() * 2];
    nums.iter().enumerate().for_each(|(ind, &x)| {
        res[ind] = x;
        res[ind + nums.len()] = x;
    });
    res
}