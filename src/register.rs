pub struct Register {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Register {
    pub fn new() -> Self {
        Register {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_af(&mut self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn get_bc(&mut self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&mut self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&mut self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn set_af(&mut self, dat: u16) {
        self.a = (dat >> 8) as u8;
        self.f = (dat & 0x00f0) as u8;
    }

    pub fn set_bc(&mut self, dat: u16) {
        self.b = (dat >> 8) as u8;
        self.c = (dat & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, dat: u16) {
        self.d = (dat >> 8) as u8;
        self.e = (dat & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, dat: u16) {
        self.h = (dat >> 8) as u8;
        self.l = (dat & 0x00ff) as u8;
    }

    pub fn get_z(&mut self) -> bool {
        ((self.f >> 7) & 0x1) != 0
    }

    pub fn get_n(&mut self) -> bool {
        ((self.f >> 6) & 0x1) != 0
    }

    pub fn get_h(&mut self) -> bool {
        ((self.f >> 5) & 0x1) != 0
    }

    pub fn get_l(&mut self) -> bool {
        ((self.f >> 4) & 0x1) != 0
    }

    pub fn set_z(&mut self, b: bool) {
        if b {
          self.f &= 0b11110000;
        } else {
          self.f &= 0b01110000;
        }
    }
    
    pub fn set_n(&mut self, b: bool) {
        if b {
          self.f &= 0b11110000;
        } else {
          self.f &= 0b10110000;
        }
    }

    pub fn set_h(&mut self, b: bool) {
        if b {
          self.f &= 0b11110000;
        } else {
          self.f &= 0b11010000;
        }
    }

    pub fn set_c(&mut self, b: bool) {
        if b {
          self.f &= 0b11110000;
        } else {
          self.f &= 0b11100000;
        }
    }
}