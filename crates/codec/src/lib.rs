// Echidna - Codec

//! Simple generic data serialization.
//! 
//! This is a replacement for `serde` that just encodes/decodes to/from a flat
//! binary dump.

pub trait Codec where Self: Sized {

    /// Decode from `buffer` into new object.
    /// 
    /// If successful, returns the number of bytes decoded and the newly
    /// created object. If not successful, returns `None`.
    fn decode(buffer: &[u8]) -> Option<(usize,Self)>;

    /// Encode `self` onto the end of `buffer`.
    /// 
    /// Returns the number of bytes appended to the buffer.
    fn encode(&self,buffer: &mut Vec<u8>) -> usize;

    /// Calculate the size of the encoded version of the object.
    /// 
    /// Returns the number of bytes this would encode into.
    fn size(&self) -> usize;
}

mod bool;
pub use crate::bool::*;

mod ui8;
pub use ui8::*;

mod ui16;
pub use ui16::*;

mod ui32;
pub use ui32::*;

mod ui64;
pub use ui64::*;

mod float;
pub use float::*;

mod string;
pub use string::*;

mod vec;
pub use vec::*;
