use std::{cell::RefCell, rc::Rc};

use crate::register::ByteRegister;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    tma_clock: u32,
    tma_period: u32,
    tma_mod: u32,
    int_flag: Rc<RefCell<ByteRegister>>,
}

impl Timer {
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tma_clock: 0,
            tma_period: 0,
            tma_mod: 0,
            int_flag: int_flag,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Timer: Unknown address."),
        }
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        match addr {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = dat,
            0xFF06 => self.tma = dat,
            0xFF07 => {
                if self.tac & 0x03 != dat & 0x03 {
                    self.tma_clock = 0;
                    self.tma_period = match dat & 0x03 {
                        0 => 1024,
                        1 => 16,
                        2 => 64,
                        3 => 256,
                        _ => panic!("Never come here"),
                    };
                    self.tima = self.tma;
                }
                self.tac = dat;
            }
            _ => panic!("Timer: Unknown address."),
        }        
    }

    pub fn run(&mut self, cycles: u32) {
        if (self.tac & 0x04) != 0 {
            let tma_cycles = self.tma_mod + cycles;
            self.tma_mod = tma_cycles % cycles;

            let n = tma_cycles / cycles;
            for _ in 0..n {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.int_flag.borrow_mut().set_bit(2, true);
                }
            }
        }
    }
}