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
        self.opcode = self.read8(self.regs.pc);
        let int_enable: u8 = self.read8(IoRegs::IE as usize);
        let mut int_flag: u8 = self.read8(IoRegs::IF as usize);
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

            self.write8(IoRegs::IF as usize, int_flag & 0xFF);
            self.ime = false;

            // ToDo: Timer cycle

            self.opcode = self.read8(self.regs.pc);
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

            // LD R16,D16
            0x02 | 0x11 | 0x21 | 0x31 => {
              let dat = self.mmc.borrow_mut().read16(self.regs.pc);
              match self.opcode {
                0x02 => self.regs.set_bc(dat),
                0x11 => self.regs.set_de(dat),
                0x21 => self.regs.set_hl(dat),
                0x31 => self.regs.sp = dat as usize,
                _ => {},
              }
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
                    0x70 => self.write8(addr as usize, self.regs.b),
                    0x71 => self.write8(addr as usize, self.regs.c),
                    0x72 => self.write8(addr as usize, self.regs.d),
                    0x73 => self.write8(addr as usize, self.regs.e),
                    0x74 => self.write8(addr as usize, self.regs.h),
                    0x75 => self.write8(addr as usize, self.regs.l),
                    0x77 => self.write8(addr as usize, self.regs.a),
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
                let addr = self.mmc.borrow_mut().read8(self.regs.get_hl() as usize);
                match self.opcode {
                    0x46 => self.regs.b = self.read8(addr as usize),
                    0x4E => self.regs.c = self.read8(addr as usize),
                    0x56 => self.regs.d = self.read8(addr as usize),
                    0x5E => self.regs.e = self.read8(addr as usize),
                    0x66 => self.regs.h = self.read8(addr as usize),
                    0x6E => self.regs.l = self.read8(addr as usize),
                    0x7E => self.regs.a = self.read8(addr as usize),
                    _ => {},
                }
            }
            _ => {},
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        self.mmc.borrow_mut().read8(addr)
    }

    pub fn write8(&mut self, addr: usize, dat: u8) {
        self.mmc.borrow_mut().write(addr, dat);
    }
}