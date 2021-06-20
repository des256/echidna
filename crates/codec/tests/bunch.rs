// Echidna - Codec - tests

//mod dump;
//use dump::*;

use codec::Codec;

#[derive(Codec)]
pub struct PublisherId {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub port: u16,
}

#[test]
fn test_bunch() {
    let source = PublisherId {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        port: 4,
    };
    let mut buffer = Vec::<u8>::new();
    source.encode(&mut buffer);

    //dump(&buffer);

    if let Some((_,target)) = PublisherId::decode(&buffer) {
        println!("{}.{}.{}.{}:{}",target.a,target.b,target.c,target.d,target.port);
        assert!(true);
    }
    else {
        assert!(false);
    }
}


/*
impl Codec for PublisherId {
    fn decode(b: &[u8]) -> Option<(usize,Self)> {
        let mut ofs = 0usize;
        let a = if let Some((l,a)) = u8::decode(&b[ofs..]) { ofs += l; a } else { return None; };
        let b = if let Some((l,b)) = u8::decode(&b[ofs..]) { ofs += l; b } else { return None; }; let c= if let Some((l,c)) = u8::decode(&b[ofs..]) { ofs += l; c } else { return None; }; let d= if let Some((l,d)) = u8::decode(&b[ofs..]) { ofs += l; d } else { return None; }; let port= if let Some((l,port)) = u16::decode(&b[ofs..]) { ofs += l; port } else { return None; };  Some((ofs,PublisherId { a: a, b: b, c: c, d: d, port: port, })) } fn encode(&self,b: &mut Vec<u8>) -> usize { let mut ofs = 0usize; ofs += self.a.encode(b); ofs += self.b.encode(b); ofs += self.c.encode(b); ofs += self.d.encode(b); ofs += self.port.encode(b); ofs } fn size(&self) -> usize { let mut ofs = 0usize; ofs += self.a.size(); ofs += self.b.size(); ofs += self.c.size(); ofs += self.d.size(); ofs += self.port.size(); ofs } }
*/