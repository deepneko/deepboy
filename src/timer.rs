use std::{cell::RefCell, rc::Rc};
use crate::register::ByteRegister;

pub struct Clock {
    pub period: u32,
    pub n: u32,
}

impl Clock {
    pub fn new(period: u32) -> Self {
        Self { period, n: 0x00 }
    }

    pub fn next(&mut self, cycles: u32) -> u32 {
        self.n += cycles;
        let s = self.n / self.period;
        self.n = self.n % self.period;
        s
    }
}

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    div_clock: Clock,
    tma_clock: Clock,
    int_flag: Rc<RefCell<ByteRegister>>,
}

impl Timer {
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_clock: Clock::new(256),
            tma_clock: Clock::new(1024),
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
            0xFF04 => {
                self.div = 0;
                self.div_clock.n = 0;
            },
            0xFF05 => self.tima = dat,
            0xFF06 => self.tma = dat,
            0xFF07 => {
                if self.tac & 0x03 != dat & 0x03 {
                    self.tma_clock.n = 0;
                    self.tma_clock.period = match dat & 0x03 {
                        0b00 => 1024,
                        0b01 => 16,
                        0b10 => 64,
                        0b11 => 256,
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
        println!("timer next div:{:x}", self.div);
        println!("timer next tima:{:x}", self.tima);
        println!("timer next tma:{:x}", self.tma);
        println!("timer next tac:{:x}", self.tac);
        println!("timer next div_clock.n:{:x}", self.div_clock.n);
        println!("timer next tma_clock.n:{:x}", self.tma_clock.n);
        self.div = self.div.wrapping_add(self.div_clock.next(cycles) as u8);

        if (self.tac & 0x04) != 0 {
            let n = self.tma_clock.next(cycles);
            for _ in 0..n {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.int_flag.borrow_mut().set_bit(2, true);
                    println!("timer next interrupt Flag::Timer");
                }
            }
        }
    }
}
