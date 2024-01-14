fn substitute_fn(x: u32) -> u8 {
     ( ( 7 * x + 7 ) % 16 ) as u8
}

fn substitute_inv_fn(x: u32) -> u8 {
    ( ( 7 * x + 63) % 16) as u8
}

fn substitute(m: u8) -> u8 {
    let bytes_lower = m >> 4; //left
    let bytes_higher = m & 15; //right

    let sub_lower = substitute_fn(bytes_lower as u32);
    let sub_higher = substitute_fn(bytes_higher as u32);

    (sub_lower << 4 ) | sub_higher
}

fn substitute_inv(m: u8) -> u8 {
    let bytes_lower = m >> 4; //left
    let bytes_higher = m & 15; //right

    let sub_lower = substitute_inv_fn(bytes_lower as u32);
    let sub_higher = substitute_inv_fn(bytes_higher as u32);

    (sub_lower << 4 ) | sub_higher
}

fn permute(m: u8) -> u8 {
    m.rotate_left(2)
}

fn permute_inv(m: u8) -> u8 {
    m.rotate_right(2)
}


fn run_rounds(m: u8, keys: &[u8]) -> u8 {
    let mut ba_block = keys[0]^m;

    for item in keys.iter().take((keys.len() - 2) + 1).skip(1) {
        ba_block = substitute(ba_block);
        ba_block = permute(ba_block);
        ba_block ^= item ;
    }

    ba_block = substitute(ba_block);
    ba_block ^= keys[keys.len() - 1];
    
    ba_block
}

fn run_rounds_rev(m: u8, keys_rev: &[u8]) -> u8 {

    let mut ba_block = keys_rev[0]^m;
    ba_block = substitute_inv(ba_block);

        for item in keys_rev.iter().take((keys_rev.len() - 2) + 1).skip(1) {
            ba_block ^= item;
            ba_block = permute_inv(ba_block);
            ba_block = substitute_inv(ba_block); 
        }

    ba_block ^= keys_rev[keys_rev.len() - 1];
    ba_block
}


pub fn enc(m: u8, k: u32) -> u8 {
    let keys = k.to_be_bytes();
    run_rounds(m, &keys)
}

pub fn dec(c: u8, k: u32) -> u8 {
    let mut keys = k.to_be_bytes();
    keys.reverse();

    run_rounds_rev(c, &keys)
}

pub fn enc_ecb(m: &mut [u8], k: u32) {
    let keys = k.to_be_bytes();

    for i in m {
        *i = run_rounds(*i, &keys);
    }
    
}

pub fn dec_ecb(c: &mut [u8], k: u32) {
    let mut keys = k.to_be_bytes();
    keys.reverse();
    for i in c {
        *i = run_rounds_rev(*i, &keys);
    }
}

pub fn enc_cbc(m: &mut [u8], k: u32, iv: u8) {
    let keys = k.to_be_bytes();
    let mut new_iv = iv;

    for i in m {
        *i ^= new_iv ;
        *i = run_rounds(*i, &keys);
        new_iv = *i;
    }
}

pub fn dec_cbc(c: &mut [u8], k: u32, iv: u8) {
    let mut keys = k.to_be_bytes();
    keys.reverse();
    
    let mut new_iv = iv;

    for i in c {
        let org_cipher = *i;
        *i = run_rounds_rev(*i, &keys);
        *i ^= new_iv;
        new_iv = org_cipher;
    }
}
