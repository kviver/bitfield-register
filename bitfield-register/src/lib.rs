// Support using Serde without the standard library!
#![cfg_attr(feature = "no_std", no_std)]

pub trait BitfieldRegister {
    type Data;
    const REGISTER_SIZE: usize;
    fn data(&self) -> & Self::Data;
}

pub trait FromBitfield<Array> {
    fn from_bitfield(array:Array) -> Self;
}

pub trait IntoBitfield<Array> {
    fn into_bitfield(self) -> Array;
}

impl FromBitfield<[u8;1]> for u8 {
    fn from_bitfield(array: [u8;1]) -> Self {
        return array[0];
    }
}

impl IntoBitfield<[u8;1]> for u8 {
    fn into_bitfield(self) -> [u8;1]{
        return [self;1];
    }
}
