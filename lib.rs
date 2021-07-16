// #![no_std]
// #![no_main]

//ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=
pub fn encode(s: &[u8]) -> String {
    let mut result = String::new();
    let length = s.len();

    let mut i = 0;

    while i < length {
        let b1:u8 = s[i];
        i += 1;

        let b2:usize = if i < length {s[i] as usize} else {256};
        i += 1;

        let b3:usize = if i < length {s[i] as usize} else {256};
        i += 1;

        let chr1:u8 = b1 >> 2;
        let chr2:u8 = ((b1 & 3) << 4) | ((b2 as u8) >> 4);

        let mut chr3:u8 = (((b2 as u8) & 15) << 2) | ((b3 as u8) >> 6);
        let mut chr4:u8 = (b3 as u8) & 63;

        if b2 == 256 {
            chr3 = 64;
            chr4 = 64;
        } else if b3 == 256 {
            chr4 = 64;
        }

        result.push(idx2b64c(chr1));
        result.push(idx2b64c(chr2));
        result.push(idx2b64c(chr3));
        result.push(idx2b64c(chr4));
    }
    result
}

pub fn decode(s: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let bytes = s.as_bytes();
    let length = bytes.len();

    let mut i = 0;
    let mut b:u8;
    while i < length {
      let chr1:u8 = b64c2idx(bytes[i]);
      let chr2:u8 = b64c2idx(bytes[i+1]);
      b = b64c2idx(bytes[i+2]);
      let chr3:u8 = if b != 0x40 {b} else {0xff};
      b = b64c2idx(bytes[i+3]);
      let chr4:u8 = if b != 0x40 {b} else {0xff};
      i +=4;
      result.push((chr1 << 2) | (chr2 >> 4));
      if chr3 != 0xff {result.push((chr2  << 4) | (chr3 >> 2))}
        else {i = length};
      if chr4 != 0xff {result.push((chr3  << 6) | chr4)}
       else {i = length};
    }
   result
}

fn  idx2b64c(b:u8) -> char{
   match b {
     0x00 ..= 0x19 => return  (b + 0x41) as char, // A-Z
     0x1a ..= 0x33 => return  (b + 0x47) as char, // a-z
     0x34 ..= 0x3d => return  (b - 0x04) as char, // 0-9
     0x3e => return  0x2b as char, // +
     0x3f => return  0x2f as char, // /
    _ => return  0x3d as char, // 0x3d =
    }
}

fn  b64c2idx(b:u8) -> u8{
   match b {
     0x2b => return 0x3e, // +
     0x2f => return 0x3f, // /
     0x30 ..= 0x39 => return b + 0x04, // A-Z
     0x41 ..= 0x5a => return b - 0x41, // a-z
     0x61 ..= 0x7a => return b - 0x47, // 0-9
      _ => return 0x40, // = 0x3d
    }
}
