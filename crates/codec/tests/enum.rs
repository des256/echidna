// Echidna - Codec - tests

//mod dump;
//use dump::*;

use codec::Codec;
use macros::codec;

#[derive(codec)]
enum MyEnum {
    One,
    Two,
    Three(f32),
    Four(f32,f32),
    Five { foo: u32, bar: f64, },
}

fn dump_myenum(e: &MyEnum) {
    match e  {
        MyEnum::One => println!("MyEnum::One"),
        MyEnum::Two => println!("MyEnum::Two"),
        MyEnum::Three(x) => println!("MyEnum::Three({})",x),
        MyEnum::Four(x,y) => println!("MyEnum::Four({},{})",x,y),
        MyEnum::Five { foo,bar } => println!("MyEnum::Five {{ foo: {}, bar: {}, }}",foo,bar),
    }
}

#[test]
fn test_enum() {

    {
        let source = MyEnum::One;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        //dump(&buffer);
        if let Some((_,target)) = MyEnum::decode(&buffer) {
            dump_myenum(&target);
        }
        else {
            assert!(false);
        }
    }

    {
        let source = MyEnum::Two;
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        //dump(&buffer);
        if let Some((_,target)) = MyEnum::decode(&buffer) {
            dump_myenum(&target);
        }
        else {
            assert!(false);
        }
    }

    {
        let source = MyEnum::Three(15.0);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        //dump(&buffer);
        if let Some((_,target)) = MyEnum::decode(&buffer) {
            dump_myenum(&target);
        }
        else {
            assert!(false);
        }
    }

    {
        let source = MyEnum::Four(16.0,17.0);
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        //dump(&buffer);
        if let Some((_,target)) = MyEnum::decode(&buffer) {
            dump_myenum(&target);
        }
        else {
            assert!(false);
        }
    }

    {
        let source = MyEnum::Five { foo: 9, bar: -19.0, };
        let mut buffer = Vec::<u8>::new();
        source.encode(&mut buffer);
        //dump(&buffer);
        if let Some((_,target)) = MyEnum::decode(&buffer) {
            dump_myenum(&target);
        }
        else {
            assert!(false);
        }
    }

    assert!(true);
}
