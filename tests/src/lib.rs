#![feature(proc_macro)]

extern crate bitfield_register;
extern crate bitfield_register_macro;


macro_rules! test_default {
    ($reg_type:ty, expected_value = $value:expr, expected_data = $data:expr) => (
        let reg = <$reg_type as Default>::default();
        assert_eq!(reg.get_field(), $value);
        assert_eq!(reg.data(), &$data);
    )
}

macro_rules! test_get {
    ($reg_type:ty, from = $from:expr, expected_value = $value:expr) => (
        let reg : $reg_type = From::from($from);
        assert_eq!(reg.get_field(), $value);
    )
}

macro_rules! test_set {
    ($reg_type:ty, from = $from:expr, value = $value:expr, expected_value = $exp_value:expr, expected_data = $data:expr) => (
        let mut reg : $reg_type = From::from($from);
        reg.set_field($value);
        assert_eq!(reg.get_field(), $exp_value);
        assert_eq!(reg.data(), &$data);
    )
}

mod bit0_test {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Bit0 {
        #[bitfield(at = 0)]
        field: u8,
    }

    #[test]
    fn bit_0_is_lsb() {
        test_default!(
            Bit0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Bit0,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Bit0,
            from = [0b1;1],
            expected_value = 1
        );

        test_get!(
            Bit0,
            from = [0b11111110;1],
            expected_value = 0
        );

        test_get!(
            Bit0,
            from = [0b11111111;1],
            expected_value = 1
        );

        test_set!(
            Bit0,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit0,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [1;1]
        );

        test_set!(
            Bit0,
            from = [0;1],
            value = 0b10,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit0,
            from = [0;1],
            value = 0b11,
            expected_value = 1,
            expected_data = [1;1]
        );

        test_set!(
            Bit0,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11111110;1]
        );

        test_set!(
            Bit0,
            from = [0b11111111;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11111111;1]
        );
    }
}

mod bit2_test {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Bit2 {
        #[bitfield(at = 2)]
        field: u8,
    }

    #[test]
    fn bit_2_test() {
        test_default!(
            Bit2,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Bit2,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Bit2,
            from = [0b00000100;1],
            expected_value = 1
        );

        test_get!(
            Bit2,
            from = [0b11111011;1],
            expected_value = 0
        );

        test_get!(
            Bit2,
            from = [0b11111111;1],
            expected_value = 1
        );

        test_set!(
            Bit2,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit2,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b00000100;1]
        );

        test_set!(
            Bit2,
            from = [0;1],
            value = 0b10,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit2,
            from = [0;1],
            value = 0b11,
            expected_value = 1,
            expected_data = [0b00000100;1]
        );

        test_set!(
            Bit2,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11111011;1]
        );

        test_set!(
            Bit2,
            from = [0b11111111;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11111111;1]
        );
    }
}

mod bit7_test {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Bit7 {
        #[bitfield(at = 7)]
        field: u8,
    }

    #[test]
    fn bit_7_is_msb() {
        test_default!(
            Bit7,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Bit7,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Bit7,
            from = [0b10000000;1],
            expected_value = 1
        );

        test_get!(
            Bit7,
            from = [0b01111111;1],
            expected_value = 0
        );

        test_get!(
            Bit7,
            from = [0b11111111;1],
            expected_value = 1
        );

        test_set!(
            Bit7,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit7,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b10000000;1]
        );

        test_set!(
            Bit7,
            from = [0;1],
            value = 0b10,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Bit7,
            from = [0;1],
            value = 0b11,
            expected_value = 1,
            expected_data = [0b10000000;1]
        );

        test_set!(
            Bit7,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b01111111;1]
        );

        test_set!(
            Bit7,
            from = [0b11111111;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11111111;1]
        );
    }
}

mod bit8_test {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Bit8 {
        #[bitfield(at = 8)]
        field: u8,
    }

    #[test]
    fn bit_8_is_lsb() {
        test_default!(
            Bit8,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_get!(
            Bit8,
            from = [0;2],
            expected_value = 0
        );

        test_get!(
            Bit8,
            from = [0,0b1],
            expected_value = 1
        );

        test_get!(
            Bit8,
            from = [0,0b11111110],
            expected_value = 0
        );

        test_get!(
            Bit8,
            from = [0,0b11111111],
            expected_value = 1
        );

        test_set!(
            Bit8,
            from = [0;2],
            value = 0,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_set!(
            Bit8,
            from = [0;2],
            value = 1,
            expected_value = 1,
            expected_data = [0,1]
        );

        test_set!(
            Bit8,
            from = [0;2],
            value = 0b10,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_set!(
            Bit8,
            from = [0;2],
            value = 0b11,
            expected_value = 1,
            expected_data = [0,1]
        );

        test_set!(
            Bit8,
            from = [0,0b11111111],
            value = 0,
            expected_value = 0,
            expected_data = [0,0b11111110]
        );

        test_set!(
            Bit8,
            from = [0,0b11111111],
            value = 1,
            expected_value = 1,
            expected_data = [0,0b11111111]
        );
    }
}

mod bit15_test {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Bit15 {
        #[bitfield(at = 15)]
        field: u8,
    }

    #[test]
    fn bit_15_is_msb() {
        test_default!(
            Bit15,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_get!(
            Bit15,
            from = [0;2],
            expected_value = 0
        );

        test_get!(
            Bit15,
            from = [0,0b10000000],
            expected_value = 1
        );

        test_get!(
            Bit15,
            from = [0,0b01111111],
            expected_value = 0
        );

        test_get!(
            Bit15,
            from = [0,0b11111111],
            expected_value = 1
        );

        test_set!(
            Bit15,
            from = [0;2],
            value = 0,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_set!(
            Bit15,
            from = [0;2],
            value = 1,
            expected_value = 1,
            expected_data = [0,0b10000000]
        );

        test_set!(
            Bit15,
            from = [0;2],
            value = 0b10,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_set!(
            Bit15,
            from = [0;2],
            value = 0b11,
            expected_value = 1,
            expected_data = [0,0b10000000]
        );

        test_set!(
            Bit15,
            from = [0,0b11111111],
            value = 0,
            expected_value = 0,
            expected_data = [0,0b01111111]
        );

        test_set!(
            Bit15,
            from = [0,0b11111111],
            value = 1,
            expected_value = 1,
            expected_data = [0,0b11111111]
        );
    }
}

mod field_0_0 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Field00 {
        #[bitfield(from = 0, to = 0)]
        field: u8,
    }

    #[test]
    fn field_0_0_test() {
        test_default!(
            Field00,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Field00,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Field00,
            from = [0b1;1],
            expected_value = 1
        );

        test_get!(
            Field00,
            from = [0b11111110;1],
            expected_value = 0
        );

        test_get!(
            Field00,
            from = [0b11111111;1],
            expected_value = 1
        );

        test_set!(
            Field00,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field00,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b1;1]
        );

        test_set!(
            Field00,
            from = [0b11111110;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11111110;1]
        );

        test_set!(
            Field00,
            from = [0b11111110;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field00,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11111110;1]
        );

        test_set!(
            Field00,
            from = [0b11111111;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11111111;1]
        );
    }
}

mod field_0_3 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Field03 {
        #[bitfield(from = 0, to = 3)]
        field: u8,
    }

    #[test]
    fn field_0_3_test() {
        test_default!(
            Field03,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Field03,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Field03,
            from = [0b1;1],
            expected_value = 1
        );

        test_get!(
            Field03,
            from = [0b1111;1],
            expected_value = 0b1111
        );

        test_get!(
            Field03,
            from = [0b11110000;1],
            expected_value = 0
        );

        test_get!(
            Field03,
            from = [0b11110001;1],
            expected_value = 0b1
        );

        test_get!(
            Field03,
            from = [0b11111111;1],
            expected_value = 0b1111
        );

        test_set!(
            Field03,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field03,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b1;1]
        );

        test_set!(
            Field03,
            from = [0;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b1111;1]
        );

        test_set!(
            Field03,
            from = [0;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b1111;1]
        );

        test_set!(
            Field03,
            from = [0b11110000;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11110000;1]
        );

        test_set!(
            Field03,
            from = [0b11110000;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11110001;1]
        );

        test_set!(
            Field03,
            from = [0b11110000;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field03,
            from = [0b11110000;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field03,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11110000;1]
        );

        test_set!(
            Field03,
            from = [0b11111111;1],
            value = 0b1,
            expected_value = 0b1,
            expected_data = [0b11110001;1]
        );

        test_set!(
            Field03,
            from = [0b11111111;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field03,
            from = [0b11111111;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );
    }
}

mod field_2_5 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Field25 {
        #[bitfield(from = 2, to = 5)]
        field: u8,
    }

    #[test]
    fn field_2_5_test() {
        test_default!(
            Field25,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Field25,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Field25,
            from = [0b100;1],
            expected_value = 1
        );

        test_get!(
            Field25,
            from = [0b111100;1],
            expected_value = 0b1111
        );

        test_get!(
            Field25,
            from = [0b11000011;1],
            expected_value = 0
        );

        test_get!(
            Field25,
            from = [0b11000111;1],
            expected_value = 0b1
        );

        test_get!(
            Field25,
            from = [0b11111111;1],
            expected_value = 0b1111
        );

        test_set!(
            Field25,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field25,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b100;1]
        );

        test_set!(
            Field25,
            from = [0;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b111100;1]
        );

        test_set!(
            Field25,
            from = [0;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b111100;1]
        );

        test_set!(
            Field25,
            from = [0b11000011;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11000011;1]
        );

        test_set!(
            Field25,
            from = [0b11000011;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b11000111;1]
        );

        test_set!(
            Field25,
            from = [0b11000011;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field25,
            from = [0b11000011;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field25,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b11000011;1]
        );

        test_set!(
            Field25,
            from = [0b11111111;1],
            value = 0b1,
            expected_value = 0b1,
            expected_data = [0b11000111;1]
        );

        test_set!(
            Field25,
            from = [0b11111111;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field25,
            from = [0b11111111;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );
    }
}

mod field_4_7 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Field47 {
        #[bitfield(from = 4, to = 7)]
        field: u8,
    }

    #[test]
    fn field_4_7_test() {
        test_default!(
            Field47,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Field47,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Field47,
            from = [0b10000;1],
            expected_value = 1
        );

        test_get!(
            Field47,
            from = [0b11110000;1],
            expected_value = 0b1111
        );

        test_get!(
            Field47,
            from = [0b00001111;1],
            expected_value = 0
        );

        test_get!(
            Field47,
            from = [0b00011111;1],
            expected_value = 0b1
        );

        test_get!(
            Field47,
            from = [0b11111111;1],
            expected_value = 0b1111
        );

        test_set!(
            Field47,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field47,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b10000;1]
        );

        test_set!(
            Field47,
            from = [0;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11110000;1]
        );

        test_set!(
            Field47,
            from = [0;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11110000;1]
        );

        test_set!(
            Field47,
            from = [0b00001111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b00001111;1]
        );

        test_set!(
            Field47,
            from = [0b00001111;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b00011111;1]
        );

        test_set!(
            Field47,
            from = [0b00001111;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field47,
            from = [0b00001111;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field47,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0b00001111;1]
        );

        test_set!(
            Field47,
            from = [0b11111111;1],
            value = 0b1,
            expected_value = 0b1,
            expected_data = [0b00011111;1]
        );

        test_set!(
            Field47,
            from = [0b11111111;1],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field47,
            from = [0b11111111;1],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;1]
        );
    }
}

mod field_0_7 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    #[register()]
    struct Field07 {
        #[bitfield(from = 0, to = 7)]
        field: u8,
    }

    #[test]
    fn field_0_7_test() {
        test_default!(
            Field07,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_get!(
            Field07,
            from = [0;1],
            expected_value = 0
        );

        test_get!(
            Field07,
            from = [0b1;1],
            expected_value = 1
        );

        test_get!(
            Field07,
            from = [0b11111111;1],
            expected_value = 0b11111111
        );

        test_set!(
            Field07,
            from = [0;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field07,
            from = [0;1],
            value = 1,
            expected_value = 1,
            expected_data = [0b1;1]
        );

        test_set!(
            Field07,
            from = [0;1],
            value = 0b11111111,
            expected_value = 0b11111111,
            expected_data = [0b11111111;1]
        );

        test_set!(
            Field07,
            from = [0b11111111;1],
            value = 0,
            expected_value = 0,
            expected_data = [0;1]
        );

        test_set!(
            Field07,
            from = [0b11111111;1],
            value = 0b1,
            expected_value = 0b1,
            expected_data = [0b00000001;1]
        );

        test_set!(
            Field07,
            from = [0b11111111;1],
            value = 0b11111111,
            expected_value = 0b11111111,
            expected_data = [0b11111111;1]
        );
    }
}

mod field_6_9 {
    use super::bitfield_register::BitfieldRegister;
    use super::bitfield_register_macro::register;

    // [0 1 2 3 4 5 6 7] [8 9 10 11 12 13 14 15]
    // [0 1 2 3 4 5 6 7] [0 1  2  3  4  5  6  7]

    #[register()]
    struct Field69 {
        #[bitfield(from = 6, to = 9)]
        field: u8,
    }

    #[test]
    fn field_6_9_test() {
        test_default!(
            Field69,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_get!(
            Field69,
            from = [0;2],
            expected_value = 0
        );

        test_get!(
            Field69,
            from = [0b1000000,0],
            expected_value = 1
        );

        test_get!(
            Field69,
            from = [0b11000000,0b11],
            expected_value = 0b1111
        );

        test_get!(
            Field69,
            from = [0b00111111,0b11111100],
            expected_value = 0
        );

        test_get!(
            Field69,
            from = [0b01111111, 0b11111100],
            expected_value = 0b1
        );

        test_get!(
            Field69,
            from = [0b11111111,0b11111111],
            expected_value = 0b1111
        );

        test_set!(
            Field69,
            from = [0;2],
            value = 0,
            expected_value = 0,
            expected_data = [0;2]
        );

        test_set!(
            Field69,
            from = [0;2],
            value = 1,
            expected_value = 1,
            expected_data = [0b01000000,0]
        );

        test_set!(
            Field69,
            from = [0;2],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11000000,0b00000011]
        );

        test_set!(
            Field69,
            from = [0;2],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11000000,0b00000011]
        );

        test_set!(
            Field69,
            from = [0b00111111,0b11111100],
            value = 0,
            expected_value = 0,
            expected_data = [0b00111111,0b11111100]
        );

        test_set!(
            Field69,
            from = [0b00111111,0b11111100],
            value = 1,
            expected_value = 1,
            expected_data = [0b01111111,0b11111100]
        );

        test_set!(
            Field69,
            from = [0b00111111,0b11111100],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111,0b11111111]
        );

        test_set!(
            Field69,
            from = [0b00111111,0b11111100],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111,0b11111111]
        );

        test_set!(
            Field69,
            from = [0b11111111;2],
            value = 0,
            expected_value = 0,
            expected_data = [0b00111111,0b11111100]
        );

        test_set!(
            Field69,
            from = [0b11111111;2],
            value = 0b1,
            expected_value = 0b1,
            expected_data = [0b01111111,0b11111100]
        );

        test_set!(
            Field69,
            from = [0b11111111;2],
            value = 0b1111,
            expected_value = 0b1111,
            expected_data = [0b11111111;2]
        );

        test_set!(
            Field69,
            from = [0b11111111;2],
            value = 0b11111,
            expected_value = 0b1111,
            expected_data = [0b11111111;2]
        );
    }
}
