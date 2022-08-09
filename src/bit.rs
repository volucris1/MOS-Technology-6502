pub trait Bit {
    /// Get bit at position
    fn get_bit(&self, bit_position: u8) -> bool;
    /// Set bit at position to 1 or 0 (true or false)
    fn set_bit(&mut self, bit_position: u8, bit_value: bool);
}

impl Bit for u8 {
    fn get_bit(&self, bit_position: u8) -> bool {
        (self & (1 << bit_position)) != 0
    }

    fn set_bit(&mut self, bit_position: u8, bit_value: bool) {
        let bit = 1 << bit_position;

        if bit_value {
            *self |= bit;
        } else {
            *self &= !bit;
        }
    }
}

#[cfg(test)]
mod test_trait_bit {
    use crate::bit::Bit;

    #[test]
    fn method_get() {
        let number = 0b1010_1001_u8;

        assert!(number.get_bit(0));
        assert!(!number.get_bit(1));
        assert!(!number.get_bit(2));
        assert!(number.get_bit(3));
        assert!(!number.get_bit(4));
        assert!(number.get_bit(5));
        assert!(!number.get_bit(6));
        assert!(number.get_bit(7));
    }

    #[test]
    fn method_set() {
        let mut number = 0b000_1110_u8;

        number.set_bit(0, true);
        number.set_bit(1, false);
        number.set_bit(2, true);
        number.set_bit(3, false);
        number.set_bit(4, true);
        number.set_bit(5, false);
        number.set_bit(6, false);
        number.set_bit(7, true);

        assert_eq!(number, 0b1001_0101);
    }
}
