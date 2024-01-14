use scrypt::{enc, dec, enc_ecb, enc_cbc, dec_ecb, dec_cbc};

fn main() {
    let cleartext = String::from("hello world");
    

    let mut msg_ecb = cleartext.as_bytes().to_vec();
    let mut msg_cbc = cleartext.as_bytes().to_vec();

    let key: u32 = 0x98267351;
    let iv: u8 = 0x42;

    let m = 12_u8;
    println!("msg (in hex): {:x?}", m);

    let c = enc(m, key);
    println!("msg encrypted (in hex): {:x?}", c);


    let d = dec(m, key);
    println!("msg decrypted (in hex): {:x?}\n", d);

   

    println!("cleartext (in hex): {:x?}\n", msg_cbc);

    enc_ecb(&mut msg_ecb, key);
    enc_cbc(&mut msg_cbc, key, iv);

    
    println!("msg_cbc encrypted (in hex): {:x?}", msg_cbc);
    println!("msg_ecb encrypted (in hex): {:x?}\n", msg_ecb);

    dec_ecb(&mut msg_ecb, key);
    dec_cbc(&mut msg_cbc, key, iv);
    
    println!("msg_ecb decrypted (in hex): {:x?}", msg_ecb);
    println!("msg_cbc decrypted (in hex): {:x?}", msg_cbc);

}
