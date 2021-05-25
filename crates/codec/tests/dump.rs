// Echidna - Codec - tests

pub fn dump(buffer: &[u8]) {
    let mut ofs = 0usize;
    for _ in 0..(buffer.len() / 16) {
        print!("{:08X}:",ofs);
        for k in 0..16 {
            print!(" {:02X}",buffer[ofs + k]);
        }
        print!("\n");
        ofs += 16;
    }
    let rem = buffer.len() - ofs;
    if rem > 0 {
        print!("{:08X}:",ofs);
        for k in 0..rem {
            print!(" {:02X}",buffer[ofs + k]);
        }
        print!("\n");
    }
}
