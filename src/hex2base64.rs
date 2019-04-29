use base64;
use hex;

fn main() {
    let astr = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    if let Ok(raw_bytes) = hex::decode(astr) {
        let b64 = base64::encode(&raw_bytes);
        println!("{}", b64);
    }
    else {
        println!("err");
    }
}
