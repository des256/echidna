// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_f32() {
        let source: f32 = 1.23456;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = f32::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_f64() {
        let source: f64 = 1.23456;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = f64::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
