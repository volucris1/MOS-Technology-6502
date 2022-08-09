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
        0xA5, 0x10, //       LDA $10h
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

#[test]
fn test_ldx() {
    let program = [
        0xA2, 0x10, // LDX #10
        0xA6, 0xFA, // LDX $FA
        0xB6, 0x10, // LDX $10, Y
        0xAE, 0x34, 0x12, // LDX $1234
        0xBE, 0x12, 0x34, // LDX $3412
    ];

    let mut cpu = create_cpu_and_load_program!(program);

    macro_rules! step {
        ($result:literal, $flags:literal) => {{
            cpu.step();
            assert_eq!(cpu.registers.x(), $result);

            let flags = cpu.registers.ps().raw();
            assert_eq!(flags, $flags);
        }};
    }

    step!(0x10, 0b0011_0000);

    cpu.memory.write(0xFA, 0x50);
    step!(0x50, 0b0011_0000);

    cpu.registers.set_y(0x10);
    cpu.memory.write(0x20, 0x90);
    step!(0x90, 0b1011_0000);

    cpu.memory.write(0x1234, 0x40);
    step!(0x40, 0b0011_0000);

    cpu.registers.set_y(0x10);
    cpu.memory.write(0x3422, 0x90);
    step!(0x90, 0b1011_0000);
}

#[test]
fn test_ldy() {
    let program = [
        0xA0, 0x10, // LDY #10
        0xA4, 0x54, // LDY $54,
        0xB4, 0x10, // LDY $10, X
        0xAC, 0x01, 0x90, // LDY $9001
        0xBC, 0x10, 0x12, // LDY $1210, X
    ];

    let mut cpu = create_cpu_and_load_program!(program);

    macro_rules! step {
        ($result:literal, $flags:literal) => {{
            cpu.step();
            assert_eq!(cpu.registers.y(), $result);

            let flags = cpu.registers.ps().raw();
            assert_eq!(flags, $flags);
        }};
    }

    step!(0x10, 0b0011_0000);

    cpu.memory.write(0x54, 0x10);
    step!(0x10, 0b0011_0000);

    cpu.registers.set_x(0x05);
    cpu.memory.write(0x15, 0x12);
    step!(0x12, 0b0011_0000);

    cpu.memory.write(0x9001, 0x90);
    step!(0x90, 0b1011_0000);

    cpu.registers.set_x(0x90);
    cpu.memory.write(0x12A0, 0x10);
    step!(0x10, 0b0011_0000);
}
