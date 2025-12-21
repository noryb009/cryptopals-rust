use anyhow::Result;

fn encode_byte(c: u8) -> u8 {
    let c = c & 0x3f;
    match c {
        0..=25 => b'A' + c,
        26..=51 => b'a' + (c - 26),
        52..=61 => b'0' + (c - 52),
        62 => b'+',
        63 => b'/',
        _ => panic!("Out of bounds after mask"),
    }
}

fn decode_char(c: u8) -> Result<u8> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok((c - b'a') + 26),
        b'0'..=b'9' => Ok((c - b'0') + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => anyhow::bail!("Not a base64 character: {}", c),
    }
}

pub fn decode(base64: &str) -> Result<Vec<u8>> {
    let mut base64 = base64.as_bytes();
    // Trim up to 3 `=`s from the string.
    for _ in 0..3 {
        match base64.strip_suffix(&[b'=']) {
            Some(b) => base64 = b,
            None => break,
        }
    }

    let mut bin: Vec<u8> = vec![];

    let (chunks, remainder) = base64.as_chunks::<4>();
    bin.reserve_exact(chunks.len() * 3 + 3);

    fn transform_values(a0: u8, a1: u8, a2: u8, a3: u8) -> [u8; 3] {
        [
            (a0 << 2) | (a1 >> 4),
            (a1 << 4) | (a2 >> 2),
            (a2 << 6) | (a3),
        ]
    }

    for c in chunks {
        let chrs = c.map(decode_char);

        let [a0, a1, a2, a3] = chrs;
        bin.extend(transform_values(a0?, a1?, a2?, a3?));
    }
    let r: Result<Vec<_>, _> = remainder
        .into_iter()
        .map(|c: &u8| decode_char(*c))
        .collect();
    match r?[..] {
        [] => (),
        [_] => anyhow::bail!("Invalid base64 length"),
        [a0, a1] => bin.extend(&transform_values(a0, a1, 0, 0)[0..1]),
        [a0, a1, a2] => bin.extend(&transform_values(a0, a1, a2, 0)[0..2]),
        _ => panic!("as_chunks remainder too large"),
    }

    return Ok(bin);
}

pub fn encode(bin: &[u8]) -> String {
    let mut bytes: Vec<u8> = vec![];
    bytes.reserve_exact((bin.len() + 2) / 3 * 4);
    let (chunks, remainder) = bin.as_chunks::<3>();

    fn transform_bytes(c0: u8, c1: u8, c2: u8) -> [u8; 4] {
        [
            encode_byte(c0 >> 2),
            encode_byte(c0 << 4 | c1 >> 4),
            encode_byte(c1 << 2 | c2 >> 6),
            encode_byte(c2),
        ]
    }

    // Encode each chunk.
    bytes.extend(
        chunks
            .into_iter()
            .flat_map(|c| transform_bytes(c[0], c[1], c[2])),
    );
    match remainder[..] {
        [] => (),
        [c0] => {
            bytes.extend(&transform_bytes(c0, 0, 0)[0..=1]);
            bytes.extend([b'=', b'=']);
        }
        [c0, c1] => {
            bytes.extend(&transform_bytes(c0, c1, 0)[0..=2]);
            bytes.extend([b'=']);
        }
        _ => panic!("as_chunks remainder too large"),
    }
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        for (input, base64) in [
            (
                "Many hands make light work.",
                "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu",
            ),
            (
                "Many hands make light work!!",
                "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmshIQ==",
            ),
            ("123!*%@$%#()_+", "MTIzISolQCQlIygpXys="),
        ] {
            let encoded = encode(input.as_bytes());
            assert!(encoded == base64);
            let decoded = decode(&encoded).unwrap();
            assert!(input.as_bytes() == decoded);
        }
    }
}
