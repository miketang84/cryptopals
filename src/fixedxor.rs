use hex;

fn main () {
    let hex1 = "1c0111001f010100061a024b53535009181c".to_string();
    let hex2 = "686974207468652062756c6c277320657965".to_string();

    let bytes1 = hex::decode(hex1).unwrap();    // omit error handling
    let bytes2 = hex::decode(hex2).unwrap();

    let encrypted_bytes : Vec<u8> = bytes1.iter()
        .zip(bytes2.iter())
        .map(|(d, k)| d ^ k)
        .collect();

    println!("{}", hex::encode(encrypted_bytes));

}
