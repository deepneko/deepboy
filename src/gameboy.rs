use std::{cell::RefCell, rc::Rc};
use super::cpu::CPU;
use super::mmc::MMC;
use super::ppu::PPU;
use super::apu::APU;
use super::timer::Timer;
use super::output::Output;

pub struct Gameboy {
    pub mmc: Rc<RefCell<MMC>>,
    pub cpu: CPU,
    pub ppu: PPU,
    pub apu: APU,
    pub timer: Timer,
    pub output: Output,
}

impl Gameboy {
    pub fn new(output: Output) -> Self {
        let mut mmc = Rc::new(RefCell::new(MMC::new()));
        let mut cpu = CPU::new(mmc.clone());
        let mut ppu = PPU::new();
        let mut apu = APU::new();
        let mut timer = Timer::new();

        Gameboy {
            mmc: mmc,
            cpu: cpu,
            ppu: ppu,
            apu: apu,
            timer: timer,
            output,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.mmc.borrow_mut().rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.mmc.borrow_mut().load_bootstrap(fname);
    }

    pub fn exec_frame(&mut self) {
        self.cpu.run(0);
        self.output.write_screen();
    }
}