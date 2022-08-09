const MEMORY_SIZE: u16 = u16::MAX;
pub type Word = u8;
pub type DWord = u16;

/// Memory map
/// Starts at 0x0000, ends at 0xFFFF
/// ┌───────────┬──────────────┬───────────┬───────────┬───────────┐
/// │ 0x0000    │ 0x2000       │ 0x4020    │ 0x6000    │ 0x8000    │
/// │ CPU RAM   │ IO Registers │ Expansion │ Save RAM  │ PGR ROM   │
/// │           │              │ ROM       │           │           │
/// └───────────┴──────────────┴───────────┴───────────┴───────────┘
/// 0x0000..0x2000 is RAM space
/// 0x2000..0x4020 is redirected to NES hardware: PPU, APU, Game pads, etc
/// 0x4020..0x6000 this space controlled by memory mappers on cartridges
/// 0x6000..0x8000 is reserved to RAM space on cartridge
/// 0x8000..0xFFFF is program ROM
pub struct Memory([Word; MEMORY_SIZE as usize]);

impl Memory {
    pub fn read(&self, addr: usize) -> u8 {
        self.0[addr]
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        self.0[addr] = data;
    }

    pub fn write_array(&mut self, start: usize, end: usize, data: &[u8]) {
        self.0[start..end].copy_from_slice(data);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE as usize])
    }
}
