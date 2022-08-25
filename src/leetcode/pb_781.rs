use std::collections::HashMap;

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut ans = HashMap::new();
    answers.iter().for_each(|&x| {
        if ans.contains_key(&x) {
            *ans.get_mut(&x).unwrap() += 1;
        } else {
            ans.insert(x, 1);
        }
    });
    let mut min_rabbits = 0;
    ans.iter().for_each(|(&k, &v)| {
        min_rabbits += (k+1) * (v/(k+1));
        if v % (k+1) > 0 {
            min_rabbits += (k+1);
        }
    });
    min_rabbits
}