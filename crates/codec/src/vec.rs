// Echidna - Codec

use crate::*;

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

#[cfg(test)]
mod tests {

    use crate::Codec;

    #[test]
    fn test_i32_vec() {
        let source: Vec<i32> = vec![-2,-1,0,1,2];
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = Vec::<i32>::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
