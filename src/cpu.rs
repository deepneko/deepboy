use crate::gameboy::Gameboy;
use super::register::Register;
use super::instruction as inst;

pub struct CPU {
    pub gameboy: Gameboy,
    pub regs: Register,
    pub opcode: u32,
    pub cycles: u32,
}

impl CPU {
    pub fn new(gameboy: Gameboy) -> Self {
        CPU {
            gameboy: gameboy,
            regs: Register::new(),
            opcode: 0,
            cycles: 0,
        }
    }

    pub fn run(&mut self, cycles: u32) {
    }

    pub fn read(&mut self, addr: u32) {
        self.gameboy.mmc.read(addr);
    }
}