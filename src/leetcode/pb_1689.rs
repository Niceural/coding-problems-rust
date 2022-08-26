pub fn min_partitions(n: String) -> i32 {
    let mut res: u32 = 0;
    for c in n.chars() {
        let num: u32 = c.to_digit(10).expect("expected a digit");
        if res < num {
            res = num;
        }
    }
    res as i32
}