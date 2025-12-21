use anyhow::Result;

use crate::hex;

pub fn xor_binary(a: &[u8], b: &[u8]) -> Result<Vec<u8>> {
    anyhow::ensure!(a.len() == b.len());
    Ok(a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>())
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
}
