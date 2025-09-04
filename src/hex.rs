use anyhow::Result;

fn char_to_nibble(c: u8) -> Result<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => anyhow::bail!("Not a hex character: {}", c),
    }
}

pub fn hex_to_bin(hex: &str) -> Result<Vec<u8>> {
    let hex = hex.as_bytes();
    anyhow::ensure!(hex.len() % 2 == 0);
    hex.chunks_exact(2).into_iter().map(|v| {
        let v1 = char_to_nibble(v[0])?;
        let v2 = char_to_nibble(v[1])?;
        Ok((v1 << 4) | v2)
    }).collect::<Result<Vec<u8>>>()
}

pub fn nibble_to_char(c: u8) -> u8 {
    let c = c & 0x0f;
    match c {
        0..=9 => c + b'0',
        10..=15 => c - 10 + b'a',
        _ => panic!("Out of bounds after mask"),
    }
}

pub fn bin_to_hex(bin: &[u8]) -> String {
    let bytes = bin.into_iter().flat_map(|b| {
        [nibble_to_char(*b >> 4), nibble_to_char(*b)]
    }).collect::<Vec<u8>>();
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let input = "0123456789abcdef";
        let v1 = hex_to_bin(input).unwrap();
        assert!(v1 == vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef));
        assert!(input == bin_to_hex(&v1));
    }

    #[test]
    fn test_upper_hex() {
        let input = "0123456789ABCDEF";
        let v1 = hex_to_bin(input).unwrap();
        assert!(v1 == vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef));
    }

    #[test]
    fn test_errors() {
        assert!(hex_to_bin("0ab").is_err());
        assert!(hex_to_bin("0g").is_err());
        assert!(hex_to_bin("0-").is_err());
        assert!(hex_to_bin("0^").is_err());
        assert!(hex_to_bin("\0\0").is_err());
    }
}
