use super::flags::Flags;

pub type ProgramCounter = u16;
pub type StackPointer = u16;
pub type Accumulator = u8;
pub type IndexRegister = u8;
pub type ProcessorStatus = u8;

pub struct Registers {
    a: Accumulator,
    x: IndexRegister,
    y: IndexRegister,
    ps: Flags,
    pc: ProgramCounter,
    sp: StackPointer,
}

impl Registers {
    pub fn a(&self) -> Accumulator {
        self.a
    }

    pub fn set_a(&mut self, a: Accumulator) {
        self.a = a;
    }

    pub fn x(&self) -> IndexRegister {
        self.x
    }

    pub fn set_x(&mut self, x: IndexRegister) {
        self.x = x;
    }

    pub fn y(&self) -> IndexRegister {
        self.y
    }

    pub fn set_y(&mut self, y: IndexRegister) {
        self.y = y;
    }

    pub fn pc(&self) -> ProgramCounter {
        self.pc
    }

    pub fn set_pc(&mut self, pc: ProgramCounter) {
        self.pc = pc
    }

    pub fn sp(&self) -> StackPointer {
        self.sp
    }

    pub fn sp_mut(&mut self) -> &mut StackPointer {
        &mut self.sp
    }

    pub fn ps(&mut self) -> &Flags {
        &self.ps
    }

    pub fn ps_mut(&mut self) -> &mut Flags {
        &mut self.ps
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0x01FF,
            pc: 0x8000,
            ps: Flags::default(),
        }
    }
}
