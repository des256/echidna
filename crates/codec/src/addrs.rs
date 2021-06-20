use {
    crate::*,
    r#async::net::{
        Ipv4Addr,
        Ipv6Addr,
        IpAddr,
        SocketAddrV4,
        SocketAddrV6,
        SocketAddr,
    },
};

impl Codec for Ipv4Addr {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        Some((4,Ipv4Addr::new(buffer[0],buffer[1],buffer[2],buffer[3])))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let addr = self.octets();
        addr[0].encode(buffer);
        addr[1].encode(buffer);
        addr[2].encode(buffer);
        addr[3].encode(buffer);
        4
    }

    fn size(&self) -> usize {
        4
    }   
}

impl Codec for Ipv6Addr {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        let a = { if let Some((_,r)) = u16::decode(&buffer[0..]) { r } else { return None; } };
        let b = { if let Some((_,r)) = u16::decode(&buffer[2..]) { r } else { return None; } };
        let c = { if let Some((_,r)) = u16::decode(&buffer[4..]) { r } else { return None; } };
        let d = { if let Some((_,r)) = u16::decode(&buffer[6..]) { r } else { return None; } };
        let e = { if let Some((_,r)) = u16::decode(&buffer[8..]) { r } else { return None; } };
        let f = { if let Some((_,r)) = u16::decode(&buffer[10..]) { r } else { return None; } };
        let g = { if let Some((_,r)) = u16::decode(&buffer[12..]) { r } else { return None; } };
        let h = { if let Some((_,r)) = u16::decode(&buffer[14..]) { r } else { return None; } };
        Some((16,Ipv6Addr::new(a,b,c,d,e,f,g,h)))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let addr = self.segments();
        addr[0].encode(buffer);
        addr[1].encode(buffer);
        addr[2].encode(buffer);
        addr[3].encode(buffer);
        addr[4].encode(buffer);
        addr[5].encode(buffer);
        addr[6].encode(buffer);
        addr[7].encode(buffer);
        16
    }

    fn size(&self) -> usize {
        16
    }   
}

impl Codec for IpAddr {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        match buffer[0] {
            0 => if let Some((_,result)) = Ipv4Addr::decode(&buffer[1..]) {
                Some((5,IpAddr::V4(result)))
            } else { None }
            1 => if let Some((_,result)) = Ipv6Addr::decode(&buffer[1..]) {
                Some((17,IpAddr::V6(result)))
            } else { None }
            _ => None,
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        match self {
            IpAddr::V4(addr) => {
                buffer.push(0);
                addr.encode(buffer);
                5
            },
            IpAddr::V6(addr) => {
                buffer.push(1);
                addr.encode(buffer);
                17
            },
        }
    }

    fn size(&self) -> usize {
        match self {
            IpAddr::V4(_) => 5,
            IpAddr::V6(_) => 17,
        }
    }   
}

impl Codec for SocketAddrV4 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        if let Some((_,port)) = u16::decode(&buffer[4..]) {
            Some((6,SocketAddrV4::new(Ipv4Addr::new(buffer[0],buffer[1],buffer[2],buffer[3]),port)))
        }
        else {
            None
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let addr = self.ip().octets();
        addr[0].encode(buffer);
        addr[1].encode(buffer);
        addr[2].encode(buffer);
        addr[3].encode(buffer);
        self.port().encode(buffer);
        6
    }

    fn size(&self) -> usize {
        6
    }   
}

impl Codec for SocketAddrV6 {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        let a = { if let Some((_,r)) = u16::decode(&buffer[0..]) { r } else { return None; } };
        let b = { if let Some((_,r)) = u16::decode(&buffer[2..]) { r } else { return None; } };
        let c = { if let Some((_,r)) = u16::decode(&buffer[4..]) { r } else { return None; } };
        let d = { if let Some((_,r)) = u16::decode(&buffer[6..]) { r } else { return None; } };
        let e = { if let Some((_,r)) = u16::decode(&buffer[8..]) { r } else { return None; } };
        let f = { if let Some((_,r)) = u16::decode(&buffer[10..]) { r } else { return None; } };
        let g = { if let Some((_,r)) = u16::decode(&buffer[12..]) { r } else { return None; } };
        let h = { if let Some((_,r)) = u16::decode(&buffer[14..]) { r } else { return None; } };
        let port = { if let Some((_,r)) = u16::decode(&buffer[16..]) { r } else { return None; } };
        Some((18,SocketAddrV6::new(Ipv6Addr::new(a,b,c,d,e,f,g,h),port,0,0)))
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        let addr = self.ip().segments();
        addr[0].encode(buffer);
        addr[1].encode(buffer);
        addr[2].encode(buffer);
        addr[3].encode(buffer);
        addr[4].encode(buffer);
        addr[5].encode(buffer);
        addr[6].encode(buffer);
        addr[7].encode(buffer);
        self.port().encode(buffer);
        18
    }

    fn size(&self) -> usize {
        18
    }   
}

impl Codec for SocketAddr {
    fn decode(buffer: &[u8]) -> Option<(usize,Self)> {
        match buffer[0] {
            0 => if let Some((_,result)) = SocketAddrV4::decode(&buffer[1..]) {
                Some((7,SocketAddr::V4(result)))
            } else { None }
            1 => if let Some((_,result)) = SocketAddrV6::decode(&buffer[1..]) {
                Some((19,SocketAddr::V6(result)))
            } else { None }
            _ => None,
        }
    }

    fn encode(&self,buffer: &mut Vec<u8>) -> usize {
        match self {
            SocketAddr::V4(addr) => {
                buffer.push(0);
                addr.encode(buffer);
                7
            },
            SocketAddr::V6(addr) => {
                buffer.push(1);
                addr.encode(buffer);
                19
            },
        }
    }

    fn size(&self) -> usize {
        match self {
            SocketAddr::V4(_) => 5,
            SocketAddr::V6(_) => 17,
        }
    }   
}

#[cfg(test)]
mod tests {

    use crate::Codec;
    use r#async::net::{
        Ipv4Addr,
        Ipv6Addr,
        IpAddr,
        SocketAddrV4,
        SocketAddrV6,
        SocketAddr,
    };

    #[test]
    fn test_ipv4addr() {
        let source = Ipv4Addr::new(192,168,1,255);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = Ipv4Addr::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_ipv6addr() {
        let source = Ipv6Addr::new(0xFDEC,0xC0BB,0xC329,0,0,0,0,1);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = Ipv6Addr::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_ipaddr() {
        let source = IpAddr::V4(Ipv4Addr::new(192,168,1,255));
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = IpAddr::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_socketaddrv4() {
        let source = SocketAddrV4::new(Ipv4Addr::new(192,168,1,255),8080);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = SocketAddrV4::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_socketaddrv6() {
        let source = SocketAddrV6::new(Ipv6Addr::new(0xFDEC,0xC0BB,0xC329,0,0,0,0,1),8080,0,0);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = SocketAddrV6::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }

    #[test]
    fn test_socketaddr() {
        let source = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,1,255),8080));
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        if let Some((_,target)) = SocketAddr::decode(&buffer) {
            assert_eq!(source,target)
        }
        else {
            assert!(false)
        }
    }
}
