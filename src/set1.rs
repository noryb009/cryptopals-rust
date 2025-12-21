use crate::base64;
use crate::english;
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

    #[test]
    fn test_challenge3() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = "Cooking MC's like a pound of bacon";
        let in_bytes = hex::decode(input).unwrap();
        let out_bytes = (0..=255)
            .map(|x| xor::xor_byte(&in_bytes, x))
            .max_by_key(|s| english::score_english(&s))
            .unwrap();
        let output = String::from_utf8(out_bytes).unwrap();
        assert_eq!(expected, output);
        //println!("{}", output);
    }

    #[test]
    fn test_challenge4() {
        let content = std::fs::read_to_string("data/4.txt").unwrap();
        let expected = "Now that the party is jumping\n";
        let out_bytes = content
            .split('\n')
            .into_iter()
            .filter(|x| x.len() > 0)
            .map(|line| hex::decode(line).unwrap())
            .flat_map(|input| (0..=255).map(move |x| xor::xor_byte(&input, x)))
            .max_by_key(|s| english::score_english(&s))
            .unwrap();

        let output = String::from_utf8(out_bytes).unwrap();
        assert_eq!(expected, output);
        //println!("{}", output);
    }

    #[test]
    fn test_challenge5() {
        let key = "ICE";
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let output_bytes = xor::xor_repeat(input.as_bytes(), key.as_bytes());
        let output = hex::encode(&output_bytes);
        //println!("{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_challenge6() {
        let key = "ICE";
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let output_bytes = xor::xor_repeat(input.as_bytes(), key.as_bytes());
        let output = hex::encode(&output_bytes);
        //println!("{}", output);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_challenge7() {
        {
            let in1 = "this is a test";
            let in2 = "wokka wokka!!!";
            let expected = 37;
            let output = english::hamming_distance(in1.as_bytes(), in2.as_bytes()).unwrap();
            assert_eq!(expected, output);
        }
        {
            // TODO
        }
    }
}
