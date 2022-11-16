/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        if n == 1 {
            return n;
        }
        let mut left = 1;
        let mut right = n;

        loop {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => right = mid - 1,
                0 => return mid,
                _ => left = mid + 1,
            }
        }
    }
}
