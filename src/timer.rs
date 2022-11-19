pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
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
            0xFF04 => self.div = dat,
            0xFF05 => self.tima = dat,
            0xFF06 => self.tma = dat,
            0xFF07 => self.tac = dat,
            _ => panic!("Timer: Unknown address."),
        }        
    }
}