use hex;
use std::char;
use std::collections::HashMap;

fn main () {
    // ref: https://en.wikipedia.org/wiki/Letter_frequency
    let mut cha_freqs: HashMap<char, f64> = HashMap::new();
    cha_freqs.insert('a', 0.08167);
    cha_freqs.insert('b', 0.01492);
    cha_freqs.insert('c', 0.02782);
    cha_freqs.insert('d', 0.04253);
    cha_freqs.insert('e', 0.12702);
    cha_freqs.insert('f', 0.02228);
    cha_freqs.insert('g', 0.02015);
    cha_freqs.insert('h', 0.06094);
    cha_freqs.insert('i', 0.06094);
    cha_freqs.insert('j', 0.00153);
    cha_freqs.insert('k', 0.00772);
    cha_freqs.insert('l', 0.04025);
    cha_freqs.insert('m', 0.02406);
    cha_freqs.insert('n', 0.06749);
    cha_freqs.insert('o', 0.07507);
    cha_freqs.insert('p', 0.01929);
    cha_freqs.insert('q', 0.00095);
    cha_freqs.insert('r', 0.05987);
    cha_freqs.insert('s', 0.06327);
    cha_freqs.insert('t', 0.09056);
    cha_freqs.insert('u', 0.02758);
    cha_freqs.insert('v', 0.00978);
    cha_freqs.insert('w', 0.02360);
    cha_freqs.insert('x', 0.00150);
    cha_freqs.insert('y', 0.01974);
    cha_freqs.insert('z', 0.00074);
    cha_freqs.insert(' ', 0.13000);

    let hex1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();

    let bytes1 = hex::decode(hex1).unwrap();    // omit error handling

    let mut max_score: f64 = 0.0;
    let mut target_msg: Vec<u8> = vec![];
    let mut target_cipher: u8 = 0;

    for cipher in 1..255 {
        let decrypted_bytes : Vec<u8> = bytes1.iter()
            .map(|d| d ^ cipher)
            .collect();

        //println!("{}", hex::encode(decrypted_bytes));
        //println!("{:?} {}", char::from_u32(cipher as u32), String::from_utf8_lossy(&decrypted_bytes));
        
        let scores = decrypted_bytes.iter().fold(0.0, |acc, x| {
            let thischar = char::from_u32(*x as u32).unwrap();
            let score = cha_freqs.get(&thischar).unwrap_or(&0.0);
            //let score = if let Some(score) = cha_freqs.get(&thischar) {
            //    *score
            //}
            //else {
            //    0.0
            //};
            acc + score
        });

        //println!("{}", scores);
        if scores > max_score {
            target_msg = decrypted_bytes;
            target_cipher = cipher;
            max_score = scores;
        }
    }


    println!("cipher: {}, msg: {}", char::from_u32(target_cipher as u32).unwrap(), String::from_utf8_lossy(&target_msg));



}
