pub fn score_english(s: &[u8]) -> i64 {
    // Check if `output` contains any non-ascii characters.
    let is_ascii = s.iter().all(|x| match *x {
        b' '..=b'~' => true,
        b'\n' => true,
        _ => false,
    });
    if !is_ascii {
        return -1;
    }
    // Count number of characters and spaces.
    s.iter()
        //.filter(|x| **x == b' ' || (b'a' <= **x && **x <= b'z') || (b'A' <= **x && **x <= b'Z'))
        .map(|x| match x {
            b' ' => 4,
            b'e' => 3,
            b'p' | b'q' | b'x' | b'y' | b'z' => -1,
            _ => 1,
        })
        .sum::<i64>()
}
