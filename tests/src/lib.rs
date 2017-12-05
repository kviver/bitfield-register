#![feature(proc_macro)]

mod tests {
    extern crate bitfield_register;
    extern crate bitfield_register_macro;

    use self::bitfield_register_macro::register;

    #[register()]
    struct Bit0Test {
        #[bitfield(at=0)]
        bit: u8,
    }

    #[test]
    fn bit_0_is_lsb() {
        let mut test = Bit0Test::default();

        assert_eq!(test.get_bit(), 0);
        assert_eq!(test.0, [0;1]);

        test.set_bit(1);

        assert_eq!(test.get_bit(), 1);
        assert_eq!(test.0, [1;1]);
    }

    #[register()]
    struct Bit7Test {
        #[bitfield(at=7)]
        bit: u8,
    }

    #[test]
    fn bit_7_is_msb() {
        let mut test = Bit7Test::default();

        assert_eq!(test.get_bit(), 0);
        assert_eq!(test.0, [0;1]);

        test.set_bit(1);

        assert_eq!(test.get_bit(), 1);
        assert_eq!(test.0, [128;1]);
    }

    #[register()]
    struct Bit8Test {
        #[bitfield(at=8)]
        bit: u8,
    }

    #[test]
    fn bit_8_is_lsb() {
        let mut test = Bit8Test::default();

        assert_eq!(test.get_bit(), 0);
        assert_eq!(test.0, [0;2]);

        test.set_bit(1);

        assert_eq!(test.get_bit(), 1);
        assert_eq!(test.0, [0,1]);
    }

    #[register()]
    struct Bit15Test {
        #[bitfield(at=15)]
        bit: u8,
    }

    #[test]
    fn bit_15_is_msb() {
        let mut test = Bit15Test::default();

        assert_eq!(test.get_bit(), 0);
        assert_eq!(test.0, [0;2]);

        test.set_bit(1);

        assert_eq!(test.get_bit(), 1);
        assert_eq!(test.0, [0,128]);
    }

    #[register()]
    struct Field00Test {
        #[bitfield(from=0, to=0)]
        field: u8,
    }

    #[test]
    fn field_0_0_is_in_lsb() {
        let mut test = Field00Test::default();

        assert_eq!(test.get_field(), 0);
        assert_eq!(test.0, [0;1]);

        test.set_field(0b1);

        assert_eq!(test.get_field(), 1);
        assert_eq!(test.0, [0b1]);

        test.set_field(0b11);

        assert_eq!(test.get_field(), 0b1);
        assert_eq!(test.0, [0b1]);

        test.0 = [0b11111111];
        assert_eq!(test.get_field(), 0b1);
        assert_eq!(test.0, [0b11111111]);
    }

    #[register()]
    struct Field03Test {
        #[bitfield(from=0, to=3)]
        field: u8,
    }

    #[test]
    fn field_0_3_is_in_lsb() {
        let mut test = Field03Test::default();

        assert_eq!(test.get_field(), 0);
        assert_eq!(test.0, [0;1]);

        test.set_field(0b1);

        assert_eq!(test.get_field(), 1);
        assert_eq!(test.0, [0b1]);

        test.set_field(0b1111);

        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b1111]);

        test.set_field(0b11111);

        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b1111]);

        test.0 = [0b11111111];
        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b11111111]);
    }

    #[register()]
    struct Field25Test {
        #[bitfield(from=2, to=5)]
        field: u8,
    }

    #[test]
    fn field_2_5_test() {
        let mut test = Field25Test::default();

        assert_eq!(test.get_field(), 0);
        assert_eq!(test.0, [0;1]);

        test.set_field(0b1);

        assert_eq!(test.get_field(), 0b1);
        assert_eq!(test.0, [0b100]);

        test.set_field(0b1111);

        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b111100]);

        test.set_field(0b11111);

        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b111100]);

        test.0 = [0b11111111];
        assert_eq!(test.get_field(), 0b1111);
        assert_eq!(test.0, [0b11111111]);
    }
}
