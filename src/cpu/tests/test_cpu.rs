use crate::cpu::CPU;

macro_rules! create_cpu_and_load_program {
    ($program:ident) => {{
        let mut cpu = CPU::default();
        cpu.load_programm(&$program);

        cpu
    }};
}

#[test]
fn test_lda() {
    let program = [
        0xA9, 0x01, //       LDA $00
        0xA9, 0x00, //       LDA $00
        0xA9, 0x80, //       LDA $80
        0xA5, 0x10, //       LDA $10
        0xB5, 0x80, //       LDA $80, X
        0xAD, 0x34, 0x12, // LDA $1234
        0xBD, 0xFF, 0xFF, // LDA $FFFF, X
        0xB9, 0xFF, 0xFF, // LDA $FFFF, Y
        0xA1, 0xFF, //       LDA ($FF,X)
        0xB1, 0xAF, //       LDA ($AF), Y
    ];

    let mut cpu = create_cpu_and_load_program!(program);

    macro_rules! step {
        ($result:literal, $flags:literal) => {{
            cpu.step();
            assert_eq!(cpu.registers.a(), $result);

            let flags = cpu.registers.ps().raw();
            assert_eq!(flags, $flags);
        }};
    }

    step!(0x01, 0b0011_0000);
    step!(0x00, 0b0011_0010);
    step!(0x80, 0b1011_0000);

    cpu.memory.write(0x0010, 0xFF);
    step!(0xFF, 0b1011_0000);

    cpu.memory.write(0x007F, 0xBC);
    cpu.registers.set_x(0xFF);
    step!(0xBC, 0b1011_0000);

    cpu.memory.write(0x1234, 0x90);
    step!(0x90, 0b1011_0000);

    cpu.registers.set_x(0x90);
    cpu.memory.write(0x8F, 0xAC);
    step!(0xAC, 0b1011_0000);

    cpu.registers.set_y(0x90);
    cpu.memory.write(0x8F, 0xFA);
    step!(0xFA, 0b1011_0000);

    cpu.registers.set_x(0x10);
    cpu.memory.write(0x0F, 0xBC);
    cpu.memory.write(0x10, 0xCB);
    cpu.memory.write(0xCBBC, 0x50);
    step!(0x50, 0b0011_0000);

    cpu.registers.set_y(0x10);
    cpu.memory.write(0xAF, 0x10);
    cpu.memory.write(0xB0, 0x11);
    cpu.memory.write(0x1120, 0x50);
    step!(0x50, 0b0011_0000);
}
