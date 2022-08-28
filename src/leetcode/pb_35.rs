pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;
        
    let mut l: usize = 0;
    let mut r: usize = nums.len();
    
    while l < r {
        let mid = l + (r - l) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => l = mid + 1,
            Ordering::Greater => r = mid,
        }
    }
    l as i32
}