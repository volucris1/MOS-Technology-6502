use paste::paste;

use crate::bit::Bit;

/// # Flags (processor state)
/// 7 6 _ 4 3 2 1 0
/// N V 1 B D I Z C
///
/// Negative - Set if bit 7 (MSB) is 1, because that's what bit 7 is mean in signed number (it's mean
/// negativity)
/// Overflow - Set if after adding positive number ending up with negative
/// result: 64 + 64 = -128
/// Break - Set when BRK instruction has been executed and an interrupt has been generated to
/// process it (by default set to 1)
/// Decimal mode - if flag set processor will be use BCD arithmetic during addition and
/// subtraction
/// Interrupt - if flags is set processor will be not respond to interrupts from other devices
/// Zero - result of last operation is zero (x == 0)
/// Carry - overflow from bit 7
#[derive(Debug)]
pub struct Flags(u8);

macro_rules! impl_flag_getter {
    ($name:ident, $flag_position:literal) => {
        paste! {
            impl Flags {
                pub fn $name(&self) -> bool {
                    self.0.get_bit($flag_position)
                }

                pub fn [<set_ $name>](&mut self, $name: bool) {
                    self.0.set_bit($flag_position, $name)
                }
            }
        }
    };
}

impl_flag_getter!(negative, 7);
impl_flag_getter!(overflow, 6);
impl_flag_getter!(brk, 4);
impl_flag_getter!(decimical, 3);
impl_flag_getter!(interrup, 2);
impl_flag_getter!(zero, 1);
impl_flag_getter!(carry, 0);

impl Flags {
    pub fn raw(&self) -> u8 {
        self.0
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self(0b0011_0000)
    }
}
