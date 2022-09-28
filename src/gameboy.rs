use std::{cell::RefCell, rc::Rc};
use super::cpu::CPU;
use super::mmc::MMC;
use super::timer::Timer;
use super::output::Output;

pub struct Gameboy {
    pub mmc: Rc<RefCell<MMC>>,
    pub cpu: CPU,
    pub elapsed_cycles: u32,
    pub timer: Timer,
    pub output: Output,
}

impl Gameboy {
    pub fn new() -> Self {
        let mmc = Rc::new(RefCell::new(MMC::new()));
        let cpu = CPU::new(mmc.clone());
        let output = Output::new(mmc.clone());
        let timer = Timer::new();

        Gameboy {
            mmc: mmc,
            cpu: cpu,
            elapsed_cycles: 0,
            timer: timer,
            output,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.mmc.borrow_mut().rom.load(fname);
    }

    // Not used
    pub fn load_bootstrap(&mut self, fname: &String) {
        self.mmc.borrow_mut().load_bootstrap(fname);
    }

    pub fn exec_frame(&mut self) {
        let cycles = self.cpu.run();
        self.elapsed_cycles += cycles;
        self.mmc.borrow_mut().ppu.run(cycles);

        self.output.write_screen();
    }
}