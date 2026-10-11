use crate::gb::cpu::CPU;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FlagBits{
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

const OPCODES: [fn(&mut CPU) -> u8; 256] = [
// 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    nop_00, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x00
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x10
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x20
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x30
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x40
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x50
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x60
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x70
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x80
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0x90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xA0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xB0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xC0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xD0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xE0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, // 0xF0
];

fn todo(cpu: &mut CPU) -> u8 {
    todo!();
}

pub fn execute(cpu: &mut CPU) -> u8 {
    let op_index = cpu.fetch();

    OPCODES[op_index as usize](cpu)
}

fn nop_00(_cpu: &mut CPU) -> u8 {
    1
}