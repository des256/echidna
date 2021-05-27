# ECHIDNA - CODEC

Data serialization/deserialization library with very flat encoding, compiling much faster than `serde`.

## Usage

Apply the `codec` derive macro on any struct, tuple, enum, vector, string, or basic data type.

```
#[derive(codec)]
struct MyStruct {
    :
}
```

In order to encode the data onto the back of a buffer:

```
let mut buffer = Vec::<u8>::new();
let size = data.encode(&mut buffer);
```

And to synthesize the data again from an encoded buffer:

```
let result = MyStruct::decode(&buffer);
if let Some((size,data)) = result {
    :
}
else {
    panic!("data buffer corrupt");
}
```
