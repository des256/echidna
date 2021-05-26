// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_u16() {
        let source: u16 = 16384;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = u16::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_i16() {
        let source: i16 = -4096;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = i16::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
