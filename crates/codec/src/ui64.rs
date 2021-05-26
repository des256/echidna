// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_u64() {
        let source: u64 = 9999999999;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = u64::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_i64() {
        let source: i64 = -1234567890;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = i64::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
