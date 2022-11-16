pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<u8> = Vec::with_capacity(s.len() / 2);
    for b in s.bytes() {
        match b {
            123 => stack.push(123),
            91 => stack.push(91),
            40 => stack.push(40),
            41 | 93 | 125 if Some(b) != stack.pop() => return false,
            _ => (),
        }

        // if b == 123 || b == 91 || b == 40 {
        //     stack.push(b);
        // } else {
        //     match stack.pop() {
        //         Some(actual) => {
        //             if actual == 40 {
        //                 if b != 41 { return false; }
        //             } else if actual == 91 {
        //                 if b != 93 { return false; }
        //             } else {
        //                 if b != 125 { return false; }
        //             }
        //         },
        //         None => return false,
        //     }
        // }
    }

    stack.is_empty()
}