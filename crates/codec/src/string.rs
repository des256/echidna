// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_string() {
        let source = "Hello, World!".to_string();
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = String::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
