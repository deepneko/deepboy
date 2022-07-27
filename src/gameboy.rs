use std::{cell::RefCell, rc::Rc};
use super::rom::Rom;
use super::cpu::CPU;
use super::mmc::MMC;
use super::output::Output;

pub struct Gameboy {
    pub rom: Rom,
    pub cpu: CPU,
    pub mmc: MMC,
    pub output: Output,
}

impl Gameboy {
    pub fn new(output: Output) -> Self {
        let mut rom = Rom::new();
        let mut mmc = MMC::new(Rc::new(RefCell::new(rom)));
        let mut cpu = CPU::new(Rc::new(RefCell::new(mmc)));
        Gameboy {
            rom: rom,
            mmc: mmc,
            cpu: cpu,
            output,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.rom.load_bootstrap(fname);
    }

    pub fn exec_frame(&mut self) {
        self.cpu.run(0);
        self.output.write_screen();
    }
}