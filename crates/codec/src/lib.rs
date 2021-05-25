// Echidna - Codec

pub trait Codec where Self: Sized {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)>;
    fn encode(&self,buffer: &mut Vec<u8>) -> usize;
    fn size(&self) -> usize;
}

impl Codec for bool {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((1,buffer[0] != 0))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(if *self { 1 } else { 0 });
        1
    }

    fn size(&self) -> usize {
        1
    }
}

impl Codec for u8 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((1,buffer[0]))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self);
        1
    }

    fn size(&self) -> usize {
        1
    }
}

impl Codec for i8 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((1,buffer[0] as i8))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        1
    }

    fn size(&self) -> usize {
        1
    }
}

impl Codec for u16 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((2,
            (buffer[0] as u16) |
            ((buffer[1] as u16) << 8)
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((self >> 8) as u8);
        2
    }

    fn size(&self) -> usize {
        2
    }
}

impl Codec for i16 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((2,
            (
                (buffer[0] as u16) |
                ((buffer[1] as u16) << 8)
            ) as i16
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((self >> 8) as u8);
        2
    }

    fn size(&self) -> usize {
        2
    }
}

impl Codec for u32 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((4,
            (buffer[0] as u32) |
            ((buffer[1] as u32) << 8) |
            ((buffer[2] as u32) << 16) |
            ((buffer[3] as u32) << 24)
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((*self >> 8) as u8);
        buffer.push((*self >> 16) as u8);
        buffer.push((*self >> 24) as u8);
        4
    }

    fn size(&self) -> usize {
        4
    }
}

impl Codec for i32 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((4,
            (
                (buffer[0] as u32) |
                ((buffer[1] as u32) << 8) |
                ((buffer[2] as u32) << 16) |
                ((buffer[3] as u32) << 24)
            ) as i32
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((*self >> 8) as u8);
        buffer.push((*self >> 16) as u8);
        buffer.push((*self >> 24) as u8);
        4
    }

    fn size(&self) -> usize {
        4
    }
}

impl Codec for u64 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((8,
            (buffer[0] as u64) |
            ((buffer[1] as u64) << 8) |
            ((buffer[2] as u64) << 16) |
            ((buffer[3] as u64) << 24) |
            ((buffer[4] as u64) << 32) |
            ((buffer[5] as u64) << 40) |
            ((buffer[6] as u64) << 48) |
            ((buffer[7] as u64) << 56)
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((*self >> 8) as u8);
        buffer.push((*self >> 16) as u8);
        buffer.push((*self >> 24) as u8);
        buffer.push((*self >> 32) as u8);
        buffer.push((*self >> 40) as u8);
        buffer.push((*self >> 48) as u8);
        buffer.push((*self >> 56) as u8);
        8
    }

    fn size(&self) -> usize {
        8
    }   
}

impl Codec for i64 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((8,
            (
                (buffer[0] as u64) |
                ((buffer[1] as u64) << 8) |
                ((buffer[2] as u64) << 16) |
                ((buffer[3] as u64) << 24) |
                ((buffer[4] as u64) << 32) |
                ((buffer[5] as u64) << 40) |
                ((buffer[6] as u64) << 48) |
                ((buffer[7] as u64) << 56)
            ) as i64
        ))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        buffer.push(*self as u8);
        buffer.push((*self >> 8) as u8);
        buffer.push((*self >> 16) as u8);
        buffer.push((*self >> 24) as u8);
        buffer.push((*self >> 32) as u8);
        buffer.push((*self >> 40) as u8);
        buffer.push((*self >> 48) as u8);
        buffer.push((*self >> 56) as u8);
        8
    }

    fn size(&self) -> usize {
        8
    }   
}

impl Codec for f32 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        if let Some(a) = u32::decode(buffer) {
            Some((4,f32::from_bits(a.1)))
        }
        else {
            None
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let a = self.to_bits();
        a.encode(buffer)
    }

    fn size(&self) -> usize {
        4
    }
}

impl Codec for f64 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        if let Some(a) = u64::decode(buffer) {
            Some((8,f64::from_bits(a.1)))
        }
        else {
            None
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let a = self.to_bits();
        a.encode(buffer)
    }

    fn size(&self) -> usize {
        8
    }
}

impl Codec for String {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        let a = String::from_utf8_lossy(buffer);
        Some((a.len(),a.to_string()))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let slice = self.as_bytes();
        let len = slice.len();
        buffer.extend_from_slice(slice);
        len
    }

    fn size(&self) -> usize {
        let slice = self.as_bytes();
        slice.len()
    }
}

impl<T: Codec> Codec for Vec<T> {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        if let Some((_,len)) = u32::decode(buffer) {
            let mut r = Vec::<T>::new();
            let mut ofs = 4usize;
            for _ in 0..len {
                if let Some((l,item)) = T::decode(&buffer[ofs..]) {
                    ofs += l;
                    r.push(item);    
                }
                else {
                    return None;
                }
            }
            Some((ofs,r))
        }
        else {
            None
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        (self.len() as u32).encode(buffer);
        let mut len = 4;
        for item in self {
            len += item.encode(buffer);
        }
        len
    }

    fn size(&self) -> usize {
        let mut len = 4;
        for item in self {
            len += item.size();
        }
        len
    }
}
