// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_u8() {
        let source: u8 = 127;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = u8::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_i8() {
        let source: i8 = -100;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = i8::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
