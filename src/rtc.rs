use std::{cell::RefCell, rc::Rc, time, thread};
use crate::{cpu::CPU, mmc::MMC, defs::*};

pub struct RTC {
    pub cpu: CPU,
    step_cycles: u32,
    step_zero: time::Instant,
    step_flip: bool,
}

impl RTC {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        let cpu = CPU::new(mmc);
        RTC {
            cpu: cpu,
            step_cycles: 0,
            step_zero: time::Instant::now(),
            step_flip: false,
        }
    }

    pub fn set_debug(&mut self) {
        self.cpu.set_debug();
    }

    pub fn run(&mut self) -> u32 {
        if self.step_cycles > STEP_CYCLES {
            self.step_flip = true;
            self.step_cycles -= STEP_CYCLES;
            let now = time::Instant::now();
            let d = now.duration_since(self.step_zero);
            let s = u64::from(STEP_TIME.saturating_sub(d.as_millis() as u32));
            thread::sleep(time::Duration::from_millis(s));
            self.step_zero = self
                .step_zero
                .checked_add(time::Duration::from_millis(u64::from(STEP_TIME)))
                .unwrap();

            if now.checked_duration_since(self.step_zero).is_some() {
                self.step_zero = now;
            }
        }
        let cycles = self.cpu.run();
        self.step_cycles += cycles;
        cycles
    }
}