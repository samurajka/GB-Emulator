use crate::gb::{instructions::{self}, registers};

pub struct  CPU {
    pub registers: registers::Registers,
    pub ime: bool,
    pub opcode: u8,
    pub cycles: u64,
}

impl CPU {
    pub fn fetch(&mut self) -> u8 {
        let val = self.read_ram(self.registers.get_pc());
        self.registers.inc_pc();
        val
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let val = ((high as u16) << 8) | (low as u16);
        val
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        todo!()
    }

    pub fn write_ram(&mut self, addr: u16, val: u8){
        todo!()
    }
}