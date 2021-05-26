// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_u32() {
        let source: u32 = 262144;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = u32::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_i32() {
        let source: i32 = -100000;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = i32::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
