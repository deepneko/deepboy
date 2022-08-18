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

pub enum IoRegs {
    IE = 0xFFFF,
    IF = 0xFF0F,
    JOYP = 0xFF00,
    DIV = 0xFF04,
    TIMA = 0xFF05,
    TMA = 0xFF06,
    TAC = 0xFF07,
    LCDC = 0xFF40,
    TAT = 0xFF41,
    RSCY = 0xFF42,
    SCX = 0xFF43,
    LY = 0xFF44,
    LYC = 0xFF45,
    WY = 0xFF4A,
    WX = 0xFF4B,
    DMA = 0xFF46,
    BGP = 0xFF47,
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    NR10 = 0xFF10,
    NR11 = 0xFF11,
    NR12 = 0xFF12,
    NR13 = 0xFF13,
    NR14 = 0xFF14,
    NR21 = 0xFF16,
    NR22 = 0xFF17,
    NR23 = 0xFF18,
    NR24 = 0xFF19,
    NR30 = 0xFF1A,
    NR31 = 0xFF1B,
    NR32 = 0xFF1C,
    NR33 = 0xFF1D,
    NR34 = 0xFF1E,
    NR41 = 0xFF20,
    NR42 = 0xFF21,
    NR43 = 0xFF22,
    NR44 = 0xFF23,
    NR50 = 0xFF24,
    NR51 = 0xFF25,
    NR52 = 0xFF26,
}

pub enum IntFlag {
    VBLANK = 1,
    STAT = 1 << 1,
    TIMER = 1 << 2,
    SERIAL = 1 << 3,
    JOYPAD = 1 << 4,
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

    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> u16 {
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

    pub fn get_z(&self) -> bool {
        ((self.f >> 7) & 0x1) != 0
    }

    pub fn get_n(&self) -> bool {
        ((self.f >> 6) & 0x1) != 0
    }

    pub fn get_h(&self) -> bool {
        ((self.f >> 5) & 0x1) != 0
    }

    pub fn get_c(&self) -> bool {
        ((self.f >> 4) & 0x1) != 0
    }

    pub fn set_z(&mut self, b: bool) {
        if b {
          self.f |= 0b10000000;
        } else {
          self.f &= 0b01110000;
        }
    }
    
    pub fn set_n(&mut self, b: bool) {
        if b {
          self.f |= 0b01000000;
        } else {
          self.f &= 0b10110000;
        }
    }

    pub fn set_h(&mut self, b: bool) {
        if b {
          self.f |= 0b00100000;
        } else {
          self.f &= 0b11010000;
        }
    }

    pub fn set_c(&mut self, b: bool) {
        if b {
          self.f |= 0b00010000;
        } else {
          self.f &= 0b11100000;
        }
    }
}