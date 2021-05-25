// Echidna - Codec - tests

mod dump;
use dump::*;

use codec::Codec;
use macros::codec;

#[derive(codec)]
struct MyStruct {
    yesno: bool,
    ubyte: u8,
    byte: i8,
    ushort: u16,
    short: i16,
    uint: u32,
    int: i32,
    ulong: u64,
    long: i64,
    float: f32,
    double: f64,
    stuff: Vec<i32>,
}

#[test]
fn test_struct() {
    let source = MyStruct {
        yesno: false,
        ubyte: 1,
        byte: -2,
        ushort: 3,
        short: -4,
        uint: 5,
        int: -6,
        ulong: 7,
        long: -8,
        float: 9.0,
        double: -10.0,
        stuff: vec![-2,-1,0,1,2],
    };
    let mut buffer = Vec::<u8>::new();
    source.encode(&mut buffer);

    dump(&buffer);

    if let Some((_,target)) = MyStruct::decode(&buffer) {
        println!("yesno: {}",target.yesno);
        println!("ubyte: {}",target.ubyte);
        println!("byte: {}",target.byte);
        println!("ushort: {}",target.ushort);
        println!("short: {}",target.short);
        println!("uint: {}",target.uint);
        println!("int: {}",target.int);
        println!("ulong: {}",target.ulong);
        println!("long: {}",target.long);
        println!("float: {}",target.float);
        println!("double: {}",target.double);
        println!("stuff: [{},{},{},{},{}]",target.stuff[0],target.stuff[1],target.stuff[2],target.stuff[3],target.stuff[4]);
        assert!(true);
    }
    else {
        assert!(false);
    }
}
