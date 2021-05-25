// Echidna - Codec - tests

mod dump;
use dump::*;

use codec::Codec;
use macros::codec;

#[derive(codec)]
struct MyTuple(f32,f32);

#[test]
fn test_tuple() {
    let source = MyTuple(15.0,-15.0);
    let mut buffer = Vec::<u8>::new();
    source.encode(&mut buffer);

    dump(&buffer);

    if let Some((_,target)) = MyTuple::decode(&buffer) {
        println!(".0: {}",target.0);
        println!(".1: {}",target.1);
        assert!(true);
    }
    else {
        assert!(false);
    }
}
