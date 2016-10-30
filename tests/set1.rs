extern crate cryptopals;
use self::cryptopals::binary::Binary;

#[test]
fn ch01() {

    let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b6\
              5206120706f69736f6e6f7573206d757368726f6f6d";
    let out = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzI\
               G11c2hyb29t";
    assert_eq!(Binary::from_hex(inp).unwrap().to_base64(), out);
}


#[test]
fn ch02() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let c = "746865206b696420646f6e277420706c6179";

    assert_eq!((&Binary::from_hex(a).unwrap() ^
                &Binary::from_hex(b).unwrap()).to_hex(), c);

    // test BitXorAssign
    let mut t = &mut Binary::from_hex(a).unwrap();
    t ^= &Binary::from_hex(b).unwrap();
    assert_eq!(t.to_hex(), c);
}

#[test]
fn ch03() {
    let msg = Binary::from_hex("1b37373331363f78151b7f2b783431333d78397828372\
                               d363c78373e783a393b3736").unwrap();
    let mut max_nr_alpha = 0usize;
    let mut best_plain = Binary::new();
    let mut cur_plain: Binary;
    for secret_ in 0..256 {
        let secret = secret_ as u8;
        cur_plain = &msg ^ secret;
        let nr_alpha = cur_plain.raw_data().iter()
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
