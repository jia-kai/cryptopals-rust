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

