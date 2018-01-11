// Support using Serde without the standard library!
#![cfg_attr(feature = "no_std", no_std)]

extern crate byteorder;

use byteorder::{ByteOrder, LE};

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

#[derive(Debug, PartialEq)]
struct U16LE(u16);

impl FromBitfield<[u8;2]> for U16LE {
    fn from_bitfield(array: [u8;2]) -> Self {
        return U16LE(LE::read_u16(&array));
    }
}

impl IntoBitfield<[u8;2]> for U16LE {
    fn into_bitfield(self) -> [u8;2]{
        let mut res : [u8;2] = [0;2];
        LE::write_u16(&mut res, self.0);
        return res;
    }
}

mod tests {
    use super::{FromBitfield, IntoBitfield, U16LE};

    macro_rules! test_into {
        ($value:expr, $expected_array:expr) => (
            assert_eq!(
                $value.into_bitfield(),
                $expected_array
            );
        )
    }

    macro_rules! test_from {
        ($value_type:ty, $array_len:expr, $array:expr, $expected_value:expr) => (
            assert_eq!(
                <$value_type as FromBitfield<[u8;$array_len]>>::from_bitfield($array),
                $expected_value
            );
        )
    }

    #[test]
    fn bool_test() {
        test_into!(
            false,
            [0;1]
        );
        test_into!(
            true,
            [1;1]
        );

        test_from!(
            bool, 1,
            [0;1], false
        );
        test_from!(
            bool, 1,
            [1;1], true
        );
        test_from!(
            bool, 1,
            [0b11111110;1], false
        );
        test_from!(
            bool, 1,
            [0b11111111;1], true
        );
    }

    #[test]
    fn u8_test() {
        test_into!(
            (0 as u8),
            [0;1]
        );
        test_into!(
            (1 as u8),
            [1;1]
        );
        test_into!(
            (0xFF as u8),
            [0xFF;1]
        );

        test_from!(
            u8, 1,
            [0;1], 0
        );
        test_from!(
            u8, 1,
            [1;1], 1
        );
        test_from!(
            u8, 1,
            [0xFF;1], 0xFF
        );
    }

    #[test]
    fn u16_le_test() {
        test_into!(
            U16LE(0),
            [0;2]
        );
        test_into!(
            U16LE(1),
            [1,0]
        );
        test_into!(
            U16LE(0xFFFF),
            [0xFF;2]
        );

        test_from!(
            U16LE, 2,
            [0;2], U16LE(0)
        );
        test_from!(
            U16LE, 2,
            [1,0], U16LE(1)
        );
        test_from!(
            U16LE, 2,
            [0xFF;2], U16LE(0xFFFF)
        );
    }
}