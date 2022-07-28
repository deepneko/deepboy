use std::{cell::RefCell, rc::Rc};
use super::cpu::CPU;
use super::mmc::MMC;
use super::output::Output;

pub struct Gameboy {
    pub mmc: Rc<RefCell<MMC>>,
    pub cpu: CPU,
    pub output: Output,
}

impl Gameboy {
    pub fn new(output: Output) -> Self {
        let mut mmc = Rc::new(RefCell::new(MMC::new()));
        let mut cpu = CPU::new(mmc.clone());
        Gameboy {
            mmc: mmc,
            cpu: cpu,
            output,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.mmc.borrow_mut().load_rom(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.mmc.borrow_mut().load_bootstrap(fname);
    }

    pub fn exec_frame(&mut self) {
        self.cpu.run(0);
        self.output.write_screen();
    }
}