pub fn reverse_words(s: String) -> String {
    if s.len() < 2 { return s; }
    let mut left = 0;
    let mut right = 0;
    let mut curr_pos = 0;
    let mut bts = s.as_bytes();

    loop {
        if bts[curr_pos] == 32 {
            curr_pos += 1;
            left = curr_pos;
            loop {
                if bts[curr_pos] == 32 {
                    right = curr_pos - 1;
                    reverse_array(bts, left, right);
                    curr_pos += 1;
                    break;
                }
                if bts.len()-1 <= curr_pos { return String::from_utf8(bts).unwrap(); }
                curr_pos += 1;
            }
        }
        if bts.len()-1 <= curr_pos { return String::from_utf8(bts).unwrap(); }
        curr_pos += 1;
    }
}

pub fn reverse_array(s: &mut [u8], start: usize, end: usize) {
    if end <= start { return; }
    s.swap(start, end);
    reverse_array(s, start + 1, end - 1);
}