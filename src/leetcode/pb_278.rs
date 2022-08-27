pub fn first_bad_version(&self, n: i32) -> i32 {
    let mut l: i32 = 1;
    let mut r: i32 = n;
        
    while l < r {
        let mid = l + (r - l) / 2;
        match self.isBadVersion(mid) {
            false => {
                if self.isBadVersion(mid+1) {
                    return mid+1;
                }
                l = mid+1;
            },
            true => r = mid,
        }
    }
    l
}