use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use crate::register::*;
use super::register::Register;
use super::instruction as inst;

pub struct CPU {
    pub mmc: Rc<RefCell<MMC>>,
    pub regs: Register,
    pub opcode: u8,
    pub cycles: u32,
    pub halt: bool,
    pub ime: bool,
    pub ei_delay: bool,
}

impl CPU {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        CPU {
            mmc: mmc,
            regs: Register::new(),
            opcode: 0,
            cycles: 0,
            halt: false,
            ime: false,
            ei_delay: false,
        }
    }

    pub fn run(&mut self, cycle: u32) {
        self.cycles += cycle;
        self.opcode = self.imm8();
        let int_enable: u8 = self.read8(IoRegs::IE as u16);
        let mut int_flag: u8 = self.read8(IoRegs::IF as u16);
        let interrupt: u8 = int_enable & int_flag;

        if self.halt {
            if interrupt > 0 {
                self.cycles += 4;
                return;
            }
        }

        if interrupt > 0 {
            self.halt = false;
        }

        if self.ime && interrupt > 0 {
            // ToDo: Timer cycle
            // ToDo: Timer cycle

            self.write8(self.regs.sp, (self.regs.pc >> 8) as u8);
            self.regs.sp -= 1;
            self.write8(self.regs.sp, (self.regs.pc & 0xFF) as u8);
            self.regs.sp -= 1;

            if interrupt & (IntFlag::VBLANK as u8) > 0 {
                self.regs.pc = 0x40;
                int_flag &= !(IntFlag::VBLANK as u8);
            } else if interrupt & (IntFlag::STAT as u8) > 0 {
                self.regs.pc = 0x48;
                int_flag &= !(IntFlag::STAT as u8);
            } else if interrupt & (IntFlag::TIMER as u8) > 0 {
                self.regs.pc = 0x58;
                int_flag &= !(IntFlag::SERIAL as u8);
            } else if interrupt & (IntFlag::JOYPAD as u8) > 0 {
                self.regs.pc = 0x60;
                int_flag &= !(IntFlag::JOYPAD as u8);
            } else {
                panic!("Failed to handle interrupt.");
            }

            self.write8(IoRegs::IF as u16, int_flag & 0xFF);
            self.ime = false;

            // ToDo: Timer cycle

            self.opcode = self.imm8();
            self.cycles += 20;
        }

        if self.ei_delay {
            self.ei_delay = false;
            self.ime = true;
        }

        self.regs.pc += 1;

        match self.opcode {
            // NOP
            0x00 => {},
            0x10 => {},

            // JR
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                let offset = self.imm8() as i8;
                match self.opcode {
                    0x18 => self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16,
                    0x20 => if !self.regs.get_z() {
                        self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16;
                    },
                    0x28 => if self.regs.get_z() {
                        self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16;
                    }
                    0x30 => if !self.regs.get_c() {
                        self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16;
                    }
                    0x38 => if self.regs.get_c() {
                        self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16;
                    }
                    _ => {},
                }
            }

            // LD R16,D16
            0x01 | 0x11 | 0x21 | 0x31 => {
                let dat = self.imm16();
                match self.opcode {
                    0x01 => self.regs.set_bc(dat),
                    0x11 => self.regs.set_de(dat),
                    0x21 => self.regs.set_hl(dat),
                    0x31 => self.regs.sp = dat,
                    _ => {},
                }
            }

            // LD (R16),A
            0x02 => {
                let addr = self.regs.get_bc();
                self.write8(addr, self.regs.a);
            }
            0x12 => {
                let addr = self.regs.get_de();
                self.write8(addr, self.regs.a);
            }
            0x22 => {
                let addr = self.regs.get_hl();
                self.write8(addr, self.regs.a);
                self.regs.set_hl(addr + 1);
            }
            0x32 => {
                let addr = self.regs.get_hl();
                self.write8(addr, self.regs.a);
                self.regs.set_hl(addr - 1);
            }

            // INC R16
            0x03 => {
                let dat = self.regs.get_bc() + 1;
                self.regs.set_bc(dat);
            },
            0x13 => {
                let dat = self.regs.get_de() + 1;
                self.regs.set_de(dat);
            },
            0x23 => {
                let dat = self.regs.get_hl() + 1;
                self.regs.set_hl(dat);
            }
            0x33 => self.regs.sp += 1,

            // INC R8
            0x04 => self.regs.b = self.inc(self.regs.b),
            0x14 => self.regs.d = self.inc(self.regs.d),
            0x24 => self.regs.h = self.inc(self.regs.h),
            0x34 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let inc_dat = self.inc(dat);
                self.write8(addr, inc_dat);
            }

            // DEC R8
            0x05 => self.regs.b = self.dec(self.regs.b),
            0x15 => self.regs.d = self.dec(self.regs.d),
            0x25 => self.regs.h = self.inc(self.regs.h),
            0x35 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let dec_dat = self.dec(dat);
                self.write8(addr, dec_dat);
            }

            // LD R8,D8
            0x06 => self.regs.b = self.imm8(),
            0x16 => self.regs.d = self.imm8(),
            0x26 => self.regs.h = self.imm8(),
            // LD (HL),D8
            0x36 => {
                let addr = self.regs.get_hl();
                let dat = self.imm8();
                self.write8(addr, dat);
            },

            // RLCA
            0x07 => {
                let carry = self.regs.a & 0x80 > 0;
                self.regs.a <<= 1;
                if carry { self.regs.a |= 0x1 };
                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(false);
                self.regs.set_c(carry);
            }

            // RLA
            0x17 => {
                let carry = self.regs.get_c();
                self.regs.a <<= 1;
                if carry { self.regs.a |= 0x1 };
                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(false);
                self.regs.set_c(carry);
            }

            // DAA
            0x27 => {
                let mut a = self.regs.a;
                if !self.regs.get_n() {
                    if self.regs.get_h() || (a & 0x0F) > 0x09 {
                        a.wrapping_add(0x06);
                    }
                    if self.regs.get_c() || a > 0x99 {
                        a.wrapping_add(0x60);
                    }
                } else {
                    if self.regs.get_h() {
                        a.wrapping_sub(0x06);
                    }
                    if self.regs.get_c() {
                        a.wrapping_sub(0x60);
                    }
                }

                if self.regs.get_c() || a > 0x99 {
                    self.regs.set_c(true);
                }
                self.regs.set_z(a != 0x00);
                self.regs.set_h(false);
                self.regs.a = a;
            }

            // SCF
            0x37 => {
                self.regs.set_n(false);
                self.regs.set_h(false);
                self.regs.set_c(true);
            }

            // LD (A16),SP
            0x08 => {
                let addr = self.imm16();
                self.write16(addr, self.regs.sp);
            },

            // LD A,(R16)
            0x0A => {
                let addr = self.regs.get_bc();
                self.regs.a = self.read8(addr);
            }
            0x1A => {
                let addr = self.regs.get_de();
                self.regs.a = self.read8(addr);
            }
            0x2A => {
                let addr = self.regs.get_hl();
                self.regs.a = self.read8(addr);
                self.regs.set_hl(addr + 1);
            }
            0x3A => {
                let addr = self.regs.get_hl();
                self.regs.a = self.read8(addr);
                self.regs.set_hl(addr - 1);
            }

            // LD R8,D8
            0x0E => self.regs.c = self.imm8(),
            0x1E => self.regs.e = self.imm8(),
            0x2E => self.regs.l = self.imm8(),
            0x3E => self.regs.a = self.imm8(),

            // RRCA
            0x0F => {
                let carry = self.regs.a & 0x1 > 0;
                self.regs.a <<= 1;
                if carry { self.regs.a |= 0x80 };
                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(false);
                self.regs.set_c(carry);
            }

            // RRA
            0x1F => {
                let carry = self.regs.get_c();
                self.regs.a <<= 1;
                if carry { self.regs.a |= 0x80 };
                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(false);
                self.regs.set_c(carry);
            }

            // CPL
            0x2F => {
                self.regs.a = !self.regs.a;
                self.regs.set_n(true);
                self.regs.set_h(true);
            }

            // CCF
            0x3F => {
                self.regs.set_n(false);
                self.regs.set_h(false);
                let c = self.regs.get_c();
                self.regs.set_c(!c);
            }

            // LD B,R
            0x40 => {},
            0x41 => self.regs.b = self.regs.c,
            0x42 => self.regs.b = self.regs.d,
            0x43 => self.regs.b = self.regs.e,
            0x44 => self.regs.b = self.regs.h,
            0x45 => self.regs.b = self.regs.l,
            0x47 => self.regs.b = self.regs.a,

            // LD C,R
            0x48 => self.regs.c = self.regs.b,
            0x49 => {},
            0x4A => self.regs.c = self.regs.d,
            0x4B => self.regs.c = self.regs.e,
            0x4C => self.regs.c = self.regs.h,
            0x4D => self.regs.c = self.regs.l,
            0x4F => self.regs.c = self.regs.a,

            // LD D,R
            0x50 => self.regs.d = self.regs.b,
            0x51 => self.regs.d = self.regs.c,
            0x52 => {},
            0x53 => self.regs.d = self.regs.e,
            0x54 => self.regs.d = self.regs.h,
            0x55 => self.regs.d = self.regs.l,
            0x57 => self.regs.d = self.regs.a,

            // LD E,R
            0x58 => self.regs.e = self.regs.b,
            0x59 => self.regs.e = self.regs.c,
            0x5A => self.regs.e = self.regs.d,
            0x5B => {},
            0x5C => self.regs.e = self.regs.h,
            0x5D => self.regs.e = self.regs.l,
            0x5F => self.regs.e = self.regs.a,

            // LD H,R
            0x60 => self.regs.h = self.regs.b,
            0x61 => self.regs.h = self.regs.c,
            0x62 => self.regs.h = self.regs.d,
            0x63 => self.regs.h = self.regs.e,
            0x64 => {},
            0x65 => self.regs.h = self.regs.l,
            0x67 => self.regs.h = self.regs.a,

            // LD L,R
            0x68 => self.regs.l = self.regs.b,
            0x69 => self.regs.l = self.regs.c,
            0x6A => self.regs.l = self.regs.d,
            0x6B => self.regs.l = self.regs.e,
            0x6C => self.regs.l = self.regs.h,
            0x6D => {},
            0x6F => self.regs.l = self.regs.a,

            // LD (HL),R
            0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => {
                let addr = self.regs.get_hl();
                match self.opcode {
                    0x70 => self.write8(addr, self.regs.b),
                    0x71 => self.write8(addr, self.regs.c),
                    0x72 => self.write8(addr, self.regs.d),
                    0x73 => self.write8(addr, self.regs.e),
                    0x74 => self.write8(addr, self.regs.h),
                    0x75 => self.write8(addr, self.regs.l),
                    0x77 => self.write8(addr, self.regs.a),
                    _ => {},
                }
            }

            // Halt
            0x76 => self.halt = true,

            // LD A,R
            0x78 => self.regs.a = self.regs.b,
            0x79 => self.regs.a = self.regs.c,
            0x7A => self.regs.a = self.regs.d,
            0x7B => self.regs.a = self.regs.e,
            0x7C => self.regs.a = self.regs.h,
            0x7D => self.regs.a = self.regs.l,
            0x7F => {},

            // LD R,(HL)
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => {
                let addr = self.regs.get_hl();
                match self.opcode {
                    0x46 => self.regs.b = self.read8(addr),
                    0x4E => self.regs.c = self.read8(addr),
                    0x56 => self.regs.d = self.read8(addr),
                    0x5E => self.regs.e = self.read8(addr),
                    0x66 => self.regs.h = self.read8(addr),
                    0x6E => self.regs.l = self.read8(addr),
                    0x7E => self.regs.a = self.read8(addr),
                    _ => {},
                }
            }

            // LD (A8),A
            0xE0 => {
                let addr = 0xFF00 | u16::from(self.imm8());
                self.write8(addr, self.regs.a);
            }

            // LD (C),A
            0xE2 => {
                let addr = 0xFF00 | u16::from(self.regs.c);
                self.write8(addr, self.regs.a);
            }

            // LD (A16),A
            0xEA => {
                let addr = self.imm16();
                self.write8(addr, self.regs.a);
            }

            // LD A,(A8)
            0xF0 => {
                let addr = 0xFF00 | u16::from(self.imm8());
                self.regs.a = self.read8(addr);
            }
            
            // LD A,(C)
            0xF2 => {
                let addr = 0xFF00 | u16::from(self.regs.c);
                self.regs.a = self.read8(addr);
            }

            // LD A,(A16)
            0xFA => {
                let addr = self.imm16();
                self.regs.a = self.read8(addr);
            }

            _ => {},
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        self.mmc.borrow_mut().read(addr)
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        u16::from(self.read8(addr)) | u16::from(self.read8(addr + 1)) << 8
    }

    pub fn imm8(&mut self) -> u8 {
        let ret = self.read8(self.regs.pc);
        self.regs.pc += 1;
        ret
    }

    pub fn imm16(&mut self) -> u16 {
        let ret = self.read16(self.regs.pc);
        self.regs.pc += 2;
        ret
    }

    pub fn write8(&mut self, addr: u16, dat: u8) {
        self.mmc.borrow_mut().write(addr, dat);
    }

    pub fn write16(&mut self, addr: u16, dat: u16) {
        self.write8(addr, (dat & 0xFF) as u8);
        self.write8(addr+1, (dat >> 8) as u8);
    }

    pub fn inc(&mut self, r: u8) -> u8 {
        let ret = r + 1;
        let half = (r & 0x0F) + 1;
        
        self.regs.set_z(r == 0);
        self.regs.set_n(false);
        self.regs.set_h(half & 0x10 > 0);

        ret
    }

    pub fn dec(&mut self, r: u8) -> u8 {
        let ret = r - 1;
        
        self.regs.set_z(r == 0);
        self.regs.set_n(true);
        self.regs.set_h(r & 0x0F < 1);

        ret
    }
}