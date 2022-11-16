pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut b1 = s1.as_bytes();
    let mut b2 = s2.as_bytes();
    let mut frequency_b1 = [0; 26];

    //  get the frequency of each character
    for id in 0..b1.len() {
        frequency_b1[letter_to_id(b1[id])] += 1;
    }

    for id2 in 0..b2.len() {
        if b2.len() - id2 < b1.len() { return false; }

        if 0 < frequency_b1[letter_to_id(b2[id2])] {
            let mut frequency_b2 = [0; 26];
            frequency_b2[letter_to_id(b2[id2])] += 1;
            let mut frequency_invalid = false;
            for id2 in (id2+1)..(id2+b1.len()) {
                frequency_b2[letter_to_id(b2[id2])] += 1;
                if frequency_b1[letter_to_id(b2[id2])] < frequency_b2[letter_to_id(b2[id2])] {
                    frequency_invalid = true;
                    break;
                }
            }
            if !frequency_invalid { return true; }
        }
    }

    false
}

pub fn letter_to_id(letter: u8) -> usize {
    (letter - 97) as usize
}