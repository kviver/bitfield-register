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
}
