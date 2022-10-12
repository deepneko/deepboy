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
    pub fn new(fname: &String) -> Self {
        let mmc = Rc::new(RefCell::new(MMC::new(fname)));
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

    pub fn exec_frame(&mut self) {
        let cycles = self.cpu.run();
        self.elapsed_cycles += cycles;
        self.mmc.borrow_mut().ppu.run(cycles);

        if self.mmc.borrow_mut().ppu.v_blank {
            self.output.write_screen();
            self.mmc.borrow_mut().ppu.v_blank = false;
        }
    }
}