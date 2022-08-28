pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // let mut res = Vec::with_capacity(nums.len());
    // if nums.is_empty() {
    //     return res;
    // } else if nums.last().unwrap() < &0 {
    //     for el in nums.iter().rev() {
    //         res.push(el * el);
    //     }
    // } else if nums[0] >= 0 {
    //     for el in &nums {
    //         res.push(el * el);
    //     }
    // } else {
    //     let mut prev = i32::MIN;
    //     let mut id = 0;
    //     loop {
    //         if nums[id] >= 0 && prev < 0 {
    //             break;
    //         }
    //         prev = nums[id];
    //         id += 1;
    //     }
    //     let (mut l, mut r) = (Some(id-1), Some(id)); 
    //     loop {
    //         l = match l {
    //             None => None,
    //             Some(lm) => {
    //                 if i
    //             }
    //         }
    //         if l != None && i32::abs(nums[l.unwrap()]) >= nums[r.unwrap()] {
    //             res.push(nums[l.unwrap()].pow(2));
    //             if l.unwrap() == 0 {
    //                 l = None;
    //             } else {
    //                 l += Some(1);
    //             }
    //             l = Some(l.unwrap() -= 1);
    //         } else if r != None {
    //             res.push(nums[r.unwrap()].pow(2));
    //             r += 1;
    //         }
    //     }
    // }
    
    // res
    let mut res = Vec::with_capacity(nums.len());
    for el in nums {
        res.push(el * el);
    }
    res.sort();
    res
}