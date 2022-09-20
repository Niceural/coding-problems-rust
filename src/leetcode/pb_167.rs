use core::num;

pub fn solution(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut low = 0;
    let mut high = numbers.len() - 1;

    loop {
        if target < numbers[low] + numbers[high] {
            high -= 1;
        } else if numbers[low] + numbers[high] < target {
            low += 1;
        } else {
            return vec![(low+1) as i32, (high+1) as i32];
        }

        if high <= low {
            break;
        }
    }

    vec![0, 0]
}