// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_bool() {
        let source = true;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = bool::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
