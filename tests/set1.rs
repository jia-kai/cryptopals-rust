extern crate cryptopals;
use self::cryptopals::bytearray::{ByteArray, ByteIterMaker, BinaryAlgo};
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

#[test]
fn ch01() {

    let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b6\
              5206120706f69736f6e6f7573206d757368726f6f6d";
    let out = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzI\
               G11c2hyb29t";
    assert_eq!(ByteArray::from_hex(inp).unwrap().to_base64(), out);
}


#[test]
fn ch02() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let c = "746865206b696420646f6e277420706c6179";

    assert_eq!((&ByteArray::from_hex(a).unwrap() ^
                &ByteArray::from_hex(b).unwrap()).to_hex(), c);

    // test BitXorAssign
    let mut t = ByteArray::from_hex(a).unwrap();
    t ^= &ByteArray::from_hex(b).unwrap();
    assert_eq!(t.to_hex(), c);
}

#[test]
fn ch03() {
    let msg = ByteArray::from_hex("1b37373331363f78151b7f2b783431333d78397828372\
                               d363c78373e783a393b3736").unwrap();
    let mut max_nr_alpha = 0usize;
    let mut best_plain = ByteArray::new();
    let mut cur_plain: ByteArray;
    for secret_ in 0..256 {
        let secret = secret_ as u8;
        cur_plain = &msg ^ secret;
        let nr_alpha = cur_plain.as_slice().iter()
            .filter(|x| {
                let ch = **x as char;
                ch == ' ' || ch.is_alphabetic()
            })
            .count();

        if nr_alpha > max_nr_alpha {
            max_nr_alpha = nr_alpha;
            best_plain = cur_plain;
        }
    }
    println!("max_nr_alpha={} plain={}", max_nr_alpha,
             best_plain.to_str_utf8().unwrap());
}

#[test]
fn ch04() {
    let mut reader = BufReader::new(File::open("res/ch04.txt").unwrap());
    let mut line = String::new();

    let mut max_nr_alpha = 0usize;
    let mut best_plain = ByteArray::new();
    let mut best_line_num = -1;
    let mut cur_plain: ByteArray;
    let mut line_num = 0;
    loop {
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let msg = ByteArray::from_hex(line.trim()).unwrap();
        line_num += 1;

        for secret_ in 0..256 {
            let secret = secret_ as u8;
            cur_plain = &msg ^ secret;
            let nr_alpha = cur_plain.as_slice().iter()
                .filter(|x| {
                    let ch = **x as char;
                    ch == ' ' || ch.is_alphabetic()
                })
            .count();

            if nr_alpha > max_nr_alpha {
                max_nr_alpha = nr_alpha;
                best_plain = cur_plain;
                best_line_num = line_num;
            }
        }
        line.clear();
    }
    println!("max_nr_alpha={} plain={} line={}", max_nr_alpha,
             best_plain.to_str_utf8().unwrap(), best_line_num);
}

#[test]
fn ch05() {
    let mut data = ByteArray::from_bytes(
        "Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal".to_string().into_bytes());
    data ^= ByteIterMaker::new(
        "ICE".to_string().into_bytes().into_iter().cycle());
    let ans = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d\
        63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2\
        b2027630c692b20283165286326302e27282f";
    assert_eq!(data.to_hex(), ans);
}

#[test]
fn ch06() {
    let mut data = String::new();
    File::open("res/ch06.txt").unwrap().read_to_string(&mut data).unwrap();
    let data = ByteArray::from_base64(&data).unwrap();
}
