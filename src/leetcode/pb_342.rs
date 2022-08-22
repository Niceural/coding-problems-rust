/// Powers of 4 are: 
/// 1  = 0 0 0 0 0 0 1
/// 4  = 0 0 0 0 1 0 0
/// 16 = 0 0 1 0 0 0 0
/// 64 = 1 0 0 0 0 0 0
/// 
/// The conditions are:
/// - power of 4 is always strictly positive
/// - power of 4 is power of 2
/// - power of 4 has its odd bits null
/// 
/// [C++ solution](https://leetcode.com/problems/power-of-four/discuss/2461582/C++-or-One-Liner-or-Easy-Code)
pub fn solution(n: i32) -> bool {
    n > 0 && (n & (n-1)) == 0 && (n & 0x55555555 != 0)
}