use std::{cell::RefCell, rc::Rc};

use super::cpu::CPU;
use super::mmc::MMC;
use super::output::Output;

pub struct Gameboy {
    pub mmc: Rc<RefCell<MMC>>,
    pub cpu: CPU,
    pub elapsed_cycles: u32,
    pub output: Output,
}

impl Gameboy {
    pub fn new(fname: &String) -> Self {
        let mmc = Rc::new(RefCell::new(MMC::new(fname)));
        let cpu = CPU::new(mmc.clone());
        let output = Output::new(mmc.clone());

        Gameboy {
            mmc: mmc,
            cpu: cpu,
            elapsed_cycles: 0,
            output,
        }
    }

    pub fn exec_frame(&mut self) {
        let cycles = self.cpu.run();
        self.elapsed_cycles += cycles;

        self.mmc.borrow_mut().timer.run(cycles);
        self.mmc.borrow_mut().ppu.run(cycles);
        // println!("v_blank:{}", self.mmc.borrow_mut().ppu.v_blank);
        if self.mmc.borrow_mut().ppu.v_blank {
            self.mmc.borrow_mut().ppu.v_blank = false;
            self.output.write_screen();
        }

        self.output.handle_keys();
    }
}