use scrypt::enc_ecb;

#[test]
fn test_enc_ecb() {
    let cleartext = String::from("hello world");
    let mut msg_ecb = cleartext.as_bytes().to_vec();

    let key: u32 = 0x98267351;

    enc_ecb(&mut msg_ecb, key);

    assert_eq!(msg_ecb, [0x62, 0xb0, 0x20, 0x20, 0x10, 0xc6, 0x91, 0x10, 0xc3, 0x20, 0xa0])
}