use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // other number in the pair -> index of the first number in the pair
    let mut pairs: HashMap<i32, usize> = HashMap::new();

    for (pos1, num) in nums.iter().enumerate() {
        match pairs.get(&num) {
            Some(pos2) => {
                return vec![pos1 as i32, pos2.to_owned() as i32];
            },
            None => {
                pairs.insert(target - num, pos1);
            }
        }
    }

    vec![]        
}