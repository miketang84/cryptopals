use base64;
use hex;
use hamming;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod helper;


fn main() {
    // hamming distance is the counting-ones of the xor result of two byte slice
    let mut file = File::open("6.txt").unwrap();
    let mut astr = String::new();
    file.read_to_string(&mut astr).unwrap();

    let bytes_from_base64_decoded: Vec<u8> = base64::decode(&astr.replace("\n", "")).unwrap();
    println!("{:?}", bytes_from_base64_decoded);

    let mut avr_normalized_hd_hash: HashMap<i8, f64> = HashMap::new();
    for keysize in 2..41i8 {
        // create an iterator
        let mut chunks = bytes_from_base64_decoded.chunks(keysize as usize);

        let mut sum_normalized_hdistance = 0.0;
        let mut count_times = 0;

        loop {

            match (chunks.next(), chunks.next()) {
                (Some(leftpart), Some(rightpart)) => {
                    if leftpart.len() != rightpart.len() {
                        break;
                    }

                    let hdistance = hamming::distance(leftpart, rightpart);
                    let normalized_hdistance = hdistance as f64 / keysize as f64;

                    sum_normalized_hdistance += normalized_hdistance;
                    count_times += 1;

                },
                _ => {
                    // jump out of this loop
                    break;
                }
            }
        }

        if count_times > 0 {
            let average_normalized_hdistance = sum_normalized_hdistance / count_times as f64;

            avr_normalized_hd_hash.insert(keysize, average_normalized_hdistance);

        }
        else {
            println!("keysize {} is not fit.", keysize);
        }

    }

    println!("hash {:?}", avr_normalized_hd_hash);

    let mut min_keysize = 0;
    let mut min_distance = 10000.0;
    for (k,v) in avr_normalized_hd_hash.clone() {
        if v < min_distance  {
            min_distance = v;
            min_keysize = k;
        }
    }

    println!("min {}, {}", min_keysize, min_distance);
    
    // print this to read by human to check the smallest 2 or 3 values
    let mut values: Vec<usize> = avr_normalized_hd_hash.values().map(|x|((*x)*10000.0) as usize ).collect();
    values.sort();
    println!(" values * 10000: {:?}", values);

    // divide the big vec to segments
    let segs = bytes_from_base64_decoded.len() / min_keysize as usize;
    let mut block_vec: Vec<Vec<u8>> = vec![];
    for i in 0..min_keysize {
        let mut inner_vec = vec![];
        for seg_index in 0..segs {
           inner_vec.push(bytes_from_base64_decoded[seg_index*min_keysize as usize + i as usize]);
        }
        block_vec.push(inner_vec);
    }
     
    println!("{:?}", block_vec); 

    let mut cipher_vec: Vec<char> = vec![];
    for v in block_vec {
        let cipher = helper::find_cipher(&v);
        //println!("{:?}", cipher);
        cipher_vec.push(cipher);
    }

    println!("{:?}", cipher_vec); 

    let mut cipher_byte_vec: Vec<u8> = vec![];
    for v in cipher_vec {
        cipher_byte_vec.push(v as u8);
    }

    let equal_length_cipher_vec = helper::repeat_key(&cipher_byte_vec, bytes_from_base64_decoded.len());

    let decrypted_bytes : Vec<u8> = bytes_from_base64_decoded.iter()
        .zip(equal_length_cipher_vec.iter())
        .map(|(d, k)| d ^ k)
        .collect();

    println!("{:?}", String::from_utf8_lossy(&decrypted_bytes));

}

