#![no_std]

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

// u8
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

// u16
impl FromBitfield<[u8;2]> for u16 {
    fn from_bitfield(array: [u8;2]) -> Self {
        return
            (array[0] as u16) |
            (array[1] as u16) << 8;
    }
}

impl IntoBitfield<[u8;2]> for u16 {
    fn into_bitfield(self) -> [u8;2]{
        return [
            (self & 0xFF) as u8,
            (self >> 8 & 0xFF) as u8,
        ];
    }
}


// u32 :3
impl IntoBitfield<[u8;3]> for u32 {
    fn into_bitfield(self) -> [u8;3]{
        return [
            (self & 0xFF) as u8,
            (self >> 8 & 0xFF) as u8,
            (self >> 16 & 0xFF) as u8,
        ];
    }
}

impl FromBitfield<[u8;3]> for u32 {
    fn from_bitfield(array: [u8;3]) -> Self {
        return
            (array[0] as u32)      | 
            (array[1] as u32) << 8 |
            (array[2] as u32) << 16;
    }
}

// u32 :4
impl IntoBitfield<[u8;4]> for u32 {
    fn into_bitfield(self) -> [u8;4]{
        return [
            (self & 0xFF) as u8,
            (self >> 8 & 0xFF) as u8,
            (self >> 16 & 0xFF) as u8,
            (self >> 24 & 0xFF) as u8
        ];
    }
}

impl FromBitfield<[u8;4]> for u32 {
    fn from_bitfield(array: [u8;4]) -> Self {
        return
            (array[0] as u32)       | 
            (array[1] as u32) << 8  |
            (array[2] as u32) << 16 |
            (array[3] as u32) << 24;

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

    #[test]
    fn u16_test() {
        test_into!(
            (0 as u16),
            { static X: [u8;2] = [0x00u8, 0x00u8]; X }
        );
        test_into!(
            (1 as u16),
            { static X: [u8;2] = [0x01u8, 0x00u8]; X }
        );
        test_into!(
            (0xFF as u16),
            { static X: [u8;2] = [0xFFu8, 0x00u8]; X }
        );
        test_into!(
            (0xFF00 as u16),
            { static X: [u8;2] = [0x00u8, 0xFFu8]; X }
        );
        test_into!(
            (0xFFFF as u16),
            { static X: [u8;2] = [0xFFu8, 0xFFu8]; X }
        );

        test_from!(
            u16, 2,
            { static X: [u8;2] = [0x00u8, 0x00u8]; X }, 0
        );
        test_from!(
            u16, 2,
            { static X: [u8;2] = [0x01u8, 0x00u8]; X }, 1
        );
        test_from!(
            u16, 2,
            { static X: [u8;2] = [0xFFu8, 0x00u8]; X }, 0xFF
        );
        test_from!(
            u16, 2,
            { static X: [u8;2] = [0x00u8, 0xFFu8]; X }, 0xFF00
        );
        test_from!(
            u16, 2,
            { static X: [u8;2] = [0xFFu8, 0xFFu8]; X }, 0xFFFF
        );
    }
}