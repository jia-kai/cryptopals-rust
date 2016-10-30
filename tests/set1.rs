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
