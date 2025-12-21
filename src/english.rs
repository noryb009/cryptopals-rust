pub fn score_english(s: &[u8]) -> i64 {
    // Check if `output` contains any non-ascii characters.
    let is_ascii = s.iter().all(|x| b' ' <= *x && *x <= b'~');
    if !is_ascii {
        return -1;
    }
    // Count number of characters and spaces.
    s.iter()
        .filter(|x| **x == b' ' || (b'a' <= **x && **x <= b'z'))
        .count() as i64
}
