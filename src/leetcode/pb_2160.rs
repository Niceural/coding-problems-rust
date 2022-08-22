//! ## [2160. Minimum Sum of Four Digit Number After Splitting Digits](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/)
//! In this code, array indices start 

pub fn solution(num: i32) -> i32 {
    let mut arr = [num % 10, num / 10 % 10, num / 100 % 10, num / 1000 % 10];
    arr.sort();
    10 * (arr[0] + arr[1]) + arr[2] + arr[3]
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 0 {
        _quick_sort(arr, 0, len - 1);
    }
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high; // pivot is the last value of the partition
    let mut store_index = low;
    let mut last_index = high + 1;

    loop {
        store_index += 1;
        while arr[store_index - 1] < arr[pivot] {
            store_index += 1;
        }

        last_index -= 1;
        while last_index == 0 && arr[last_index - 1] > arr[pivot] {
            last_index -= 1;
        }

        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index - 1, last_index - 1);
        }
    }

    arr.swap(store_index - 1, pivot);
    store_index - 1
}
