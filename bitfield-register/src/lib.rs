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

impl FromBitfield<[u8;1]> for bool {
    fn from_bitfield(array: [u8;1]) -> Self {
        return (array[0] & 1) != 0;
    }
}

impl IntoBitfield<[u8;1]> for bool {
    fn into_bitfield(self) -> [u8;1]{
        // If you cast a bool into an integer, true will be 1 and false will be 0.
        // https://doc.rust-lang.org/std/primitive.bool.html
        return [self as u8;1];
    }
}

mod tests {
    use super::FromBitfield;
    use super::IntoBitfield;

    #[test]
    fn bool_test() {
        assert_eq!(
            false.into_bitfield(),
            [0;1]
        );
        assert_eq!(
            true.into_bitfield(),
            [1;1]
        );
        assert_eq!(
            <bool as FromBitfield<[u8;1]>>::from_bitfield([0;1]),
            false
        );
        assert_eq!(
            <bool as FromBitfield<[u8;1]>>::from_bitfield([1;1]),
            true
        );
        assert_eq!(
            <bool as FromBitfield<[u8;1]>>::from_bitfield([2;1]),
            false
        );
    }
}