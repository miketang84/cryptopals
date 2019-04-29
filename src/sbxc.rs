use hex;
use std::char;

fn main () {
    let hex1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();

    let bytes1 = hex::decode(hex1).unwrap();    // omit error handling

    for cipher in 1..255 {
        let decrypted_bytes : Vec<u8> = bytes1.iter()
            .map(|d| d ^ cipher)
            .collect();

        //println!("{}", hex::encode(decrypted_bytes));
        println!("{:?} {}", char::from_u32(cipher as u32), String::from_utf8_lossy(&decrypted_bytes));
    }


}
