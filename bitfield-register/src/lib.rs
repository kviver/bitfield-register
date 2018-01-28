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
    use super::{FromBitfield, IntoBitfield};

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
}