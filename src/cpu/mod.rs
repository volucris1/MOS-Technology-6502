use self::registers::{Accumulator, IndexRegister, Registers};
use crate::bit::Bit;
use crate::memory::{DWord, Memory, Word};

mod flags;
mod registers;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct CPU {
    registers: Registers,
    memory: Memory,
}

impl CPU {
    pub fn load_programm(&mut self, program: &[Word]) {
        let start = 0x8000;
        self.memory
            .write_array(start, start + program.len(), program);
    }

    pub fn run(&mut self) {
        loop {
            self.step()
        }
    }

    fn step(&mut self) {
        let opcode = self.fetch_word();
        println!("{:#04X}", opcode);

        macro_rules! eval {
            // Get immediate value
            (get u8) => {
                self.fetch_word()
            };
            (get (u8)) => {{
                let addr = eval!(get u8);
                self.memory.read(addr as usize)
            }};
            (get (u8, X)) => {{
                let addr = eval!(get u8).wrapping_add(self.registers.x());
                self.memory.read(addr as usize)
            }};
            (get (u8, Y)) => {{
                let addr = eval!(get u8).wrapping_add(self.registers.y());
                self.memory.read(addr as usize)
            }};

            (get u16) => {{
                self.fetch_dword()
            }};
            (get (u16)) => {{
                let addr = eval!(get u16);
                self.memory.read(addr as usize)
            }};
            (get (u16, X)) => {{
                let addr = eval!(get u16).wrapping_add(self.registers.x() as u16);
                self.memory.read(addr as usize)
            }};
            (get (u16, Y)) => {{
                let addr = eval!(get u16).wrapping_add(self.registers.y() as u16);
                self.memory.read(addr as usize)
            }};
            (get indirect (u8, X)) => {{
                let addr_to_addr = eval!(get u8).wrapping_add(self.registers.x());
                let addr = u16::from_le_bytes([
                    self.memory.read(addr_to_addr as usize),
                    self.memory.read(addr_to_addr.wrapping_add(1) as usize)
                ]);
                self.memory.read(addr as usize)
            }};
            (get indirect (u8), Y) => {{
                let addr_to_addr = eval!(get u8);
                let addr = u16::from_le_bytes([
                    self.memory.read(addr_to_addr as usize),
                    self.memory.read(addr_to_addr.wrapping_add(1) as usize)
                ])
                .wrapping_add(self.registers.y() as u16);
                self.memory.read(addr as usize)
            }};

            (_LDA $word:ident) => {
                self.lda($word)
            };
            (LDA $arg:ident) => {{
                let word = eval!(get $arg);
                eval!(_LDA word);
            }};
            (LDA ($arg:ident)) => {{
                let word = eval!(get ($arg));
                eval!(_LDA word);
            }};
            (LDA ($arg:ident, $index_register:ident)) => {{
                let word = eval!(get ($arg, $index_register));
                eval!(_LDA word);
            }};
            (LDA indirect (u8, X)) => {{
                let word = eval!(get indirect (u8, X));
                eval!(_LDA word);
            }};
            (LDA indirect (u8), Y) => {{
                let word = eval!(get indirect (u8), Y);
                eval!(_LDA word);
            }};

            (LDX $arg:ident) => {{
                let word = eval!(get $arg);
                self.ldx(word);
            }};
            (LDX ($arg:ident)) => {{
                let word = eval!(get ($arg));
                self.ldx(word);
            }};
            (LDX ($arg:ident, $index_register:ident)) => {{
                let word = eval!(get ($arg, $index_register));
                self.ldx(word);
            }};

            (LDY $arg:ident) => {{
                let word = eval!(get $arg);
                self.ldy(word);
            }};
            (LDY ($arg:ident)) => {{
                let word = eval!(get ($arg));
                self.ldy(word);
            }};
            (LDY ($arg:ident, $index_register:ident)) => {{
                let word = eval!(get ($arg, $index_register));
                self.ldy(word);
            }};
        }

        match opcode {
            0xA9 => eval!(LDA u8),
            0xA5 => eval!(LDA(u8)),
            0xB5 => eval!(LDA(u8, X)),
            0xAD => eval!(LDA(u16)),
            0xBD => eval!(LDA(u16, X)),
            0xB9 => eval!(LDA(u16, Y)),
            0xA1 => eval!(LDA indirect (u8, X)),
            0xB1 => eval!(LDA indirect (u8), Y),

            0xA2 => eval!(LDX u8),
            0xA6 => eval!(LDX(u8)),
            0xB6 => eval!(LDX(u8, Y)),
            0xAE => eval!(LDX(u16)),
            0xBE => eval!(LDX(u16, Y)),

            0xA0 => eval!(LDY u8),
            0xA4 => eval!(LDY(u8)),
            0xB4 => eval!(LDY(u8, X)),
            0xAC => eval!(LDY(u16)),
            0xBC => eval!(LDY(u16, X)),
            _ => panic!("Unimplemented or illegal opcode: {:#04X}", opcode),
        }
    }

    fn fetch_word(&mut self) -> Word {
        let pc = self.registers.pc();
        let word = self.memory.read(pc as usize);
        self.registers.set_pc(pc + 1);

        word
    }

    fn fetch_dword(&mut self) -> DWord {
        u16::from_le_bytes([self.fetch_word(), self.fetch_word()])
    }
}

macro_rules! set_flags {
    ($flags:ident, N: $data:ident) => {{
        $flags.set_negative($data.get_bit(7))
    }};

    ($flags:ident, Z: $data:ident) => {{
        $flags.set_zero($data == 0)
    }};

    ($flags:ident, N, Z: $value:ident) => {{
        set_flags!($flags, N: $value);
        set_flags!($flags, Z: $value);
    }};
}

impl CPU {
    // Transfer Instructions
    fn lda(&mut self, a: Accumulator) {
        self.registers.set_a(a);

        let flags = self.registers.ps_mut();
        set_flags!(flags, N, Z: a);
    }

    fn ldx(&mut self, x: IndexRegister) {
        self.registers.set_x(x);

        let flags = self.registers.ps_mut();
        set_flags!(flags, N, Z: x);
    }

    fn ldy(&mut self, y: IndexRegister) {
        self.registers.set_y(y);

        let flags = self.registers.ps_mut();
        set_flags!(flags, N, Z: y);
    }
}
