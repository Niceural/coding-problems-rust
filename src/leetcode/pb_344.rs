pub fn solution(s: &mut Vec<char>) {
    if s.len() < 2 { return; }
    let mut left = 0;
    let mut right = s.len() - 1;
    
    loop {
        // let temp = s[left];
        // s[left] = s[right];
        // s[right] = temp;

        s.swap(left, right);
        
        left += 1; right -= 1;
        
        if right <= left {
            break;
        }
    }
}