// first solution
pub fn solution(nums: &Vec<i32>) {
    if nums.len() < 2 { return; }
    
    // pointer pointing to the next non zero element
    let mut non_0: usize = 0;

    for curr in 0..nums.len() {
        // find the next non zero element
        loop {
            if non_0 == nums.len() || nums[non_0] != 0 { break; }
            non_0 += 1;
        }

        // when non zero element id is out of vector, assign the remaining elements to zero
        if non_0 >= nums.len() {
            for rem in curr..nums.len() {
                nums[rem] = 0;
            }
            break;
        }

        nums[curr] = nums[non_0];
        non_0 += 1;
    }
}

pub fn other_solution(nums: &Vec<i32>) {
    if nums.len() < 2 { return; }

    let mut curr = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[curr] = nums[i];
            curr += 1;
        }
    }

    for i in curr..nums.len() {
        nums[i] = 0;
    }
}

pub fn optimal_solution(nums: &Vec<i32>) {
    if nums.len() < 2 { return; }
    let mut curr = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(curr, i);
            curr += 1;
        }
    }
}