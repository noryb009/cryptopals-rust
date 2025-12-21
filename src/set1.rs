use crate::base64;
use crate::hex;
use crate::xor;

mod tests {
    use super::*;

    #[test]
    fn test_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let raw = hex::decode(input).unwrap();
        assert_eq!(expected, base64::encode(&raw));
        //println!("{}", String::from_utf8(raw).unwrap());
    }

    #[test]
    fn test_challenge2() {
        let in1 = "1c0111001f010100061a024b53535009181c";
        let in2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let out = xor::xor_binary(&hex::decode(in1).unwrap(), &hex::decode(in2).unwrap()).unwrap();
        assert_eq!(expected, hex::encode(&out));
        //println!("{}", String::from_utf8(out).unwrap());
    }
}
