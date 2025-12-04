mod base64;
mod hex;

fn main() {
    println!("Hello, world!");
    println!("{:?}", hex::hex_to_bin("0123456789abcdef").unwrap());
    println!("{}", hex::bin_to_hex(&hex::hex_to_bin("0123456789abcdef").unwrap()));
}
