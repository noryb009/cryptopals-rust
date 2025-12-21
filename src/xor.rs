use anyhow::Result;

use crate::hex;

pub fn xor_binary(a: &[u8], b: &[u8]) -> Result<Vec<u8>> {
    anyhow::ensure!(a.len() == b.len());
    Ok(a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>())
}

pub fn xor_byte(a: &[u8], b: u8) -> Vec<u8> {
    a.into_iter().map(|x| x ^ b).collect::<Vec<u8>>()
}

pub fn xor_repeat(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.len() < b.len() {
        return xor_repeat(b, a);
    }
    a.into_iter()
        .zip(b.into_iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>()
}

mod tests {
    use super::*;

    #[test]
    fn test_xor_binary() {
        let in1 = hex::decode("00055055").unwrap();
        let in2 = hex::decode("124850fe").unwrap();
        let out = xor_binary(&in1, &in2).unwrap();
        let expected = "124d00ab";
        assert_eq!(hex::encode(&out), expected);
    }

    #[test]
    fn test_xor_byte() {
        let in1 = hex::decode("010289ff").unwrap();
        let in2 = 0x11;
        let out = xor_byte(&in1, in2);
        let expected = "101398ee";
        assert_eq!(hex::encode(&out), expected);
    }

    #[test]
    fn test_xor_repeat() {
        let in1 = hex::decode("01020304").unwrap();
        let in2 = hex::decode("0102").unwrap();
        let out = xor_repeat(&in1, &in2);
        let expected = "00000206";
        assert_eq!(hex::encode(&out), expected);
    }
}
