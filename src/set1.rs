use crate::base64;
use crate::hex;

mod tests {
    use super::*;

    #[test]
    fn test_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let raw = hex::decode(input).unwrap();
        println!("{}", String::from_utf8(raw.clone()).unwrap());
        assert!(expected == base64::encode(&raw));
    }
}
