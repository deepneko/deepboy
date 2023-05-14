use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use crate::register::*;
use super::register::Register;

pub const CYCLES: [u32; 0x100] = [
  //x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 xA xB xC xD xE xF
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0x
    1, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1x
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2x
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6x
    2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // Ax
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // Bx
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // Cx
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // Dx
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // Ex
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // Fx
];

pub const CYCLES_BRANCHED: [u32; 0x100] = [
  //x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 xA xB xC xD xE xF
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0x
    1, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1x
    3, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 2x
    3, 3, 2, 2, 3, 3, 3, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 3x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6x
    2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9x
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // Ax
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // Bx
    5, 3, 4, 4, 6, 4, 2, 4, 5, 4, 4, 0, 6, 6, 2, 4, // Cx
    5, 3, 4, 0, 6, 4, 2, 4, 5, 4, 4, 0, 6, 0, 2, 4, // Dx
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // Ex
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // Fx
];

pub const CB_CYCLES: [u32; 0x100] = [
  //x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 xA xB xC xD xE xF
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 1x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 2x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 3x
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 4x
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 5x
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 6x
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 7x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 8x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 9x
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Ax
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Bx
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Cx
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Dx
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Ex
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // Fx
];

pub struct CPU {
    pub mmc: Rc<RefCell<MMC>>,
    pub regs: Register,
    pub opcode: u8,
    pub cb_opcode: u8,
    pub halt: bool,
    pub ime: bool,
    pub ei_delay: bool,
    pub debug: bool,
}

impl CPU {
    pub fn new(mmc: Rc<RefCell<MMC>>) -> Self {
        CPU {
            mmc: mmc,
            regs: Register::new(),
            opcode: 0,
            cb_opcode: 0,
            halt: false,
            ime: true,
            ei_delay: false,
            debug: false,
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub fn run(&mut self) -> u32 {
        let hi = self.handle_interrupt();
        println!("cpu next halt:{}", self.halt);

        if hi > 0 {
            return hi;
        }

        if self.halt {
            return 1;
        }

        if self.debug {
            self.debug_out();
        }

        let mut cycles = 0;
        self.opcode = self.imm8();
        match self.opcode {
            // NOP
            0x00 | 0x10 => {},

            // NONE
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {},

            // JR
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                let offset = self.imm8();
                match self.opcode {
                    0x18 => self.jr(offset),
                    0x20 => if !self.regs.get_z() { self.jr(offset) },
                    0x28 => if self.regs.get_z() { self.jr(offset) },
                    0x30 => if !self.regs.get_c() { self.jr(offset) },
                    0x38 => if self.regs.get_c() { self.jr(offset) },
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
            0x02 => self.write8(self.regs.get_bc(), self.regs.a),
            0x12 => self.write8(self.regs.get_de(), self.regs.a),
            0x22 => {
                let addr = self.regs.get_hl();
                self.write8(addr, self.regs.a);
                self.regs.set_hl(addr.wrapping_add(1));
            }
            0x32 => {
                let addr = self.regs.get_hl();
                self.write8(addr, self.regs.a);
                self.regs.set_hl(addr.wrapping_sub(1));
            }

            // INC R16
            0x03 => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
            0x13 => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
            0x23 => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
            0x33 => self.regs.sp = self.regs.sp.wrapping_add(1),

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
            0x0C => self.regs.c = self.inc(self.regs.c),
            0x1C => self.regs.e = self.inc(self.regs.e),
            0x2C => self.regs.l = self.inc(self.regs.l),
            0x3C => self.regs.a = self.inc(self.regs.a),

            // DEC R8
            0x05 => self.regs.b = self.dec(self.regs.b),
            0x15 => self.regs.d = self.dec(self.regs.d),
            0x25 => self.regs.h = self.dec(self.regs.h),
            0x35 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let dec_dat = self.dec(dat);
                self.write8(addr, dec_dat);
            }
            0x0D => self.regs.c = self.dec(self.regs.c),
            0x1D => self.regs.e = self.dec(self.regs.e),
            0x2D => self.regs.l = self.dec(self.regs.l),
            0x3D => self.regs.a = self.dec(self.regs.a),

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
                self.regs.a = self.rlc(self.regs.a);
                self.regs.set_z(false);
            }

            // RLA
            0x17 => {
                self.regs.a = self.rl(self.regs.a);
                self.regs.set_z(false);
            }

            // DAA
            0x27 => {
                let mut a = self.regs.a;
                if !self.regs.get_n() {
                    if self.regs.get_h() || (a & 0x0F) > 0x09 {
                        a = a.wrapping_add(0x06);
                    }
                    if self.regs.get_c() || a > 0x99 {
                        a = a.wrapping_add(0x60);
                    }
                } else {
                    if self.regs.get_h() {
                        a = a.wrapping_sub(0x06);
                    }
                    if self.regs.get_c() {
                        a = a.wrapping_sub(0x60);
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

            // ADD HL,R16
            0x09 => self.add_hl(self.regs.get_bc()),
            0x19 => self.add_hl(self.regs.get_de()),
            0x29 => self.add_hl(self.regs.get_hl()),
            0x39 => self.add_hl(self.regs.sp),

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
                self.regs.set_hl(addr.wrapping_add(1));
            }
            0x3A => {
                let addr = self.regs.get_hl();
                self.regs.a = self.read8(addr);
                self.regs.set_hl(addr.wrapping_sub(1));
            }

            // DEC A,(R16)
            0x0B => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
            0x1B => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
            0x2B => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
            0x3B => self.regs.set_af(self.regs.get_af().wrapping_sub(1)),

            // LD R8,D8
            0x0E => self.regs.c = self.imm8(),
            0x1E => self.regs.e = self.imm8(),
            0x2E => self.regs.l = self.imm8(),
            0x3E => self.regs.a = self.imm8(),

            // RRCA
            0x0F => {
                self.regs.a = self.rrc(self.regs.a);
                self.regs.set_z(false);
            }

            // RRA
            0x1F => {
                self.regs.a = self.rr(self.regs.a);
                self.regs.set_z(false);
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
            0x70 => self.write8(self.regs.get_hl(), self.regs.b),
            0x71 => self.write8(self.regs.get_hl(), self.regs.c),
            0x72 => self.write8(self.regs.get_hl(), self.regs.d),
            0x73 => self.write8(self.regs.get_hl(), self.regs.e),
            0x74 => self.write8(self.regs.get_hl(), self.regs.h),
            0x75 => self.write8(self.regs.get_hl(), self.regs.l),
            0x77 => self.write8(self.regs.get_hl(), self.regs.a),

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
            0x46 => self.regs.b = self.read8(self.regs.get_hl()),
            0x4E => self.regs.c = self.read8(self.regs.get_hl()),
            0x56 => self.regs.d = self.read8(self.regs.get_hl()),
            0x5E => self.regs.e = self.read8(self.regs.get_hl()),
            0x66 => self.regs.h = self.read8(self.regs.get_hl()),
            0x6E => self.regs.l = self.read8(self.regs.get_hl()),
            0x7E => self.regs.a = self.read8(self.regs.get_hl()),

            // ADD
            0x80 => self.regs.a = self.add(self.regs.b),
            0x81 => self.regs.a = self.add(self.regs.c),
            0x82 => self.regs.a = self.add(self.regs.d),
            0x83 => self.regs.a = self.add(self.regs.e),
            0x84 => self.regs.a = self.add(self.regs.h),
            0x85 => self.regs.a = self.add(self.regs.l),
            0x86 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.add(dat);
            }
            0x87 => self.regs.a = self.add(self.regs.a),
            0xC6 => {
                let dat = self.imm8();
                self.regs.a = self.add(dat);
            }

            // ADC
            0x88 => self.regs.a = self.adc(self.regs.b),
            0x89 => self.regs.a = self.adc(self.regs.c),
            0x8A => self.regs.a = self.adc(self.regs.d),
            0x8B => self.regs.a = self.adc(self.regs.e),
            0x8C => self.regs.a = self.adc(self.regs.h),
            0x8D => self.regs.a = self.adc(self.regs.l),
            0x8E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.adc(dat);
            }
            0x8F => self.regs.a = self.adc(self.regs.a),
            0xCE => {
                let dat = self.imm8();
                self.regs.a = self.adc(dat);
            }

            // SUB
            0x90 => self.regs.a = self.sub(self.regs.b),
            0x91 => self.regs.a = self.sub(self.regs.c),
            0x92 => self.regs.a = self.sub(self.regs.d),
            0x93 => self.regs.a = self.sub(self.regs.e),
            0x94 => self.regs.a = self.sub(self.regs.h),
            0x95 => self.regs.a = self.sub(self.regs.l),
            0x96 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.sub(dat);
            }
            0x97 => self.regs.a = self.sub(self.regs.a),
            0xD6 => {
                let dat = self.imm8();
                self.regs.a = self.sub(dat);
            }

            // SBC
            0x98 => self.regs.a = self.sbc(self.regs.b),
            0x99 => self.regs.a = self.sbc(self.regs.c),
            0x9A => self.regs.a = self.sbc(self.regs.d),
            0x9B => self.regs.a = self.sbc(self.regs.e),
            0x9C => self.regs.a = self.sbc(self.regs.h),
            0x9D => self.regs.a = self.sbc(self.regs.l),
            0x9E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.sbc(dat);
            }
            0x9F => self.regs.a = self.sbc(self.regs.a),
            0xDE => {
                let dat = self.imm8();
                self.regs.a = self.sbc(dat);
            }

            // AND
            0xA0 => self.regs.a = self.and(self.regs.b),
            0xA1 => self.regs.a = self.and(self.regs.c),
            0xA2 => self.regs.a = self.and(self.regs.d),
            0xA3 => self.regs.a = self.and(self.regs.e),
            0xA4 => self.regs.a = self.and(self.regs.h),
            0xA5 => self.regs.a = self.and(self.regs.l),
            0xA6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.and(dat);
            }
            0xA7 => self.regs.a = self.and(self.regs.a),
            0xE6 => {
                let dat = self.imm8();
                self.regs.a = self.and(dat);
            }

            // XOR
            0xA8 => self.regs.a = self.xor(self.regs.b),
            0xA9 => self.regs.a = self.xor(self.regs.c),
            0xAA => self.regs.a = self.xor(self.regs.d),
            0xAB => self.regs.a = self.xor(self.regs.e),
            0xAC => self.regs.a = self.xor(self.regs.h),
            0xAD => self.regs.a = self.xor(self.regs.l),
            0xAE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.xor(dat);
            }
            0xAF => self.regs.a = self.xor(self.regs.a),
            0xEE => {
                let dat = self.imm8();
                self.regs.a = self.xor(dat);
            }

            // OR
            0xB0 => self.regs.a = self.or(self.regs.b),
            0xB1 => self.regs.a = self.or(self.regs.c),
            0xB2 => self.regs.a = self.or(self.regs.d),
            0xB3 => self.regs.a = self.or(self.regs.e),
            0xB4 => self.regs.a = self.or(self.regs.h),
            0xB5 => self.regs.a = self.or(self.regs.l),
            0xB6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.regs.a = self.or(dat);
            }
            0xB7 => self.regs.a = self.or(self.regs.a),
            0xF6 => {
                let dat = self.imm8();
                self.regs.a = self.or(dat);
            }

            // CP
            0xB8 => self.cp(self.regs.b),
            0xB9 => self.cp(self.regs.c),
            0xBA => self.cp(self.regs.d),
            0xBB => self.cp(self.regs.e),
            0xBC => self.cp(self.regs.h),
            0xBD => self.cp(self.regs.l),
            0xBE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.cp(dat);
            }
            0xBF => self.cp(self.regs.a),
            0xFE => {
                let dat = self.imm8();
                self.cp(dat);
            }

            // RET
            0xC0 => if !self.regs.get_z() { self.regs.pc = self.pop() },
            0xC8 => if self.regs.get_z() { self.regs.pc = self.pop() },
            0xC9 => self.regs.pc = self.pop(),
            0xD0 => if !self.regs.get_c() { self.regs.pc = self.pop() },
            0xD8 => if self.regs.get_c() { self.regs.pc = self.pop() },
            0xD9 => {
                self.regs.pc = self.pop();
                self.ime = true;
            }

            // POP
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                let dat = self.pop();
                match self.opcode {
                    0xC1 => self.regs.set_bc(dat),
                    0xD1 => self.regs.set_de(dat),
                    0xE1 => self.regs.set_hl(dat),
                    0xF1 => self.regs.set_af(dat),
                    _ => {},
                }
            }

            // JP
            0xC2 => if !self.regs.get_z() { self.regs.pc = self.imm16() } else { self.regs.pc += 2 },
            0xC3 => self.regs.pc = self.imm16(),
            0xCA => if self.regs.get_z() { self.regs.pc = self.imm16() } else { self.regs.pc += 2 },
            0xD2 => if !self.regs.get_c() { self.regs.pc = self.imm16() } else { self.regs.pc += 2 },
            0xDA => if self.regs.get_c() { self.regs.pc = self.imm16() } else { self.regs.pc += 2 },
            0xE9 => self.regs.pc = self.regs.get_hl(),  // For real??

            // CALL
            0xC4 => if !self.regs.get_z() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            } else {
                self.regs.pc += 2;
            }
            0xCC => if self.regs.get_z() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            } else {
                self.regs.pc += 2;
            }
            0xCD => {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            }
            0xD4 => if !self.regs.get_c() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            } else {
                self.regs.pc += 2;
            }
            0xDC => if self.regs.get_c() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            } else {
                self.regs.pc += 2;
            }

            // PUSH
            0xC5 => self.push(self.regs.get_bc()),
            0xD5 => self.push(self.regs.get_de()),
            0xE5 => self.push(self.regs.get_hl()),
            0xF5 => self.push(self.regs.get_af()),

            // RST
            0xC7 => {
                self.push(self.regs.pc);
                self.regs.pc = 0x00;
            }
            0xCF => {
                self.push(self.regs.pc);
                self.regs.pc = 0x08;
            }
            0xD7 => {
                self.push(self.regs.pc);
                self.regs.pc = 0x10;
            }
            0xDF => {
                self.push(self.regs.pc);
                self.regs.pc = 0x18;
            }
            0xE7 => {
                self.push(self.regs.pc);
                self.regs.pc = 0x20;
            }
            0xEF => {
                self.push(self.regs.pc);
                self.regs.pc = 0x28;
            }
            0xF7 => {
                self.push(self.regs.pc);
                self.regs.pc = 0x30;
            }
            0xFF => {
                self.push(self.regs.pc);
                self.regs.pc = 0x38;
            }

            // LD (A8),A
            0xE0 => {
                let addr = 0xFF00 | u16::from(self.imm8());
                // println!("0xE0 LDA, addr:{:x}", addr);
                self.write8(addr, self.regs.a);
            }

            // LD (C),A
            0xE2 => {
                let addr = 0xFF00 | u16::from(self.regs.c);
                self.write8(addr, self.regs.a);
            }

            // ADD SP,r8
            0xE8 => {
                let dat = i16::from(self.imm8() as i8) as u16;
                let half_carry = (self.regs.sp & 0xF) + (dat & 0xF) > 0xF;
                let carry = (self.regs.sp & 0xFF) + (dat & 0xFF) > 0xFF;

                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(half_carry);
                self.regs.set_c(carry);

                self.regs.sp = self.regs.sp.wrapping_add(dat);
            }

            // LD HL,SP+r8
            0xF8 => {
                let dat = i16::from(self.imm8() as i8) as u16;
                let half_carry = (self.regs.sp & 0xF) + (dat & 0xF) > 0xF;
                let carry = (self.regs.sp & 0xFF) + (dat & 0xFF) > 0xFF;

                self.regs.set_z(false);
                self.regs.set_n(false);
                self.regs.set_h(half_carry);
                self.regs.set_c(carry);

                self.regs.set_hl(self.regs.sp.wrapping_add(dat));
            }

            // LD SP,HL
            0xF9 => self.regs.sp = self.regs.get_hl(),

            // LD (A16),A
            0xEA => {
                let addr = self.imm16();
                self.write8(addr, self.regs.a);
            }

            // LD A,(A8)
            0xF0 => {
                let addr = 0xFF00 | u16::from(self.imm8());
                println!("0xF0 LDA, addr:{:x}", addr);
                self.regs.a = self.read8(addr);
            }
            
            // LD A,(C)
            0xF2 => {
                let addr = 0xFF00 | u16::from(self.regs.c);
                self.regs.a = self.read8(addr);
            }

            // DI
            0xF3 => self.ime = false,

            // LD A,(A16)
            0xFA => {
                let addr = self.imm16();
                self.regs.a = self.read8(addr);
            }

            // EI
            0xFB => self.ime = true,

            // PREFIX CB
            0xCB => self.prefix_cb(),
        }

        if self.opcode == 0xCB {
            cycles += CB_CYCLES[self.cb_opcode as usize];
        } else {
            match self.opcode {
                // JR | JP | RET | CALL
                0x20 | 0xC2 | 0xC0 | 0xC4 => cycles += self.branched_cycles(!self.regs.get_z()),
                0x28 | 0xCA | 0xC8 | 0xCC => cycles += self.branched_cycles(self.regs.get_z()),
                0x30 | 0xD2 | 0xD0 | 0xD4 => cycles += self.branched_cycles(!self.regs.get_c()),
                0x38 | 0xDA | 0xD8 | 0xDC => cycles += self.branched_cycles(self.regs.get_c()),

                _ => cycles += CYCLES[self.opcode as usize],
            }
        }

        if self.debug {
            println!("cpu cycles: {:x}", cycles)
        }
        cycles
    }

    pub fn branched_cycles(&self, cond: bool) -> u32 {
        if cond {
            CYCLES_BRANCHED[self.opcode as usize]
        } else {
            CYCLES[self.opcode as usize]
        }
    }

    pub fn prefix_cb(&mut self) {
        self.cb_opcode = self.read8(self.regs.pc);
        self.regs.pc += 1;

        match self.cb_opcode {
            // RLC
            0x00 => self.regs.b = self.rlc(self.regs.b),
            0x01 => self.regs.c = self.rlc(self.regs.c),
            0x02 => self.regs.d = self.rlc(self.regs.d),
            0x03 => self.regs.e = self.rlc(self.regs.e),
            0x04 => self.regs.h = self.rlc(self.regs.h),
            0x05 => self.regs.l = self.rlc(self.regs.l),
            0x06 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.rlc(dat);
                self.write8(addr, ret);
            }
            0x07 => self.regs.a = self.rlc(self.regs.a),

            // RRC
            0x08 => self.regs.b = self.rrc(self.regs.b),
            0x09 => self.regs.c = self.rrc(self.regs.c),
            0x0A => self.regs.d = self.rrc(self.regs.d),
            0x0B => self.regs.e = self.rrc(self.regs.e),
            0x0C => self.regs.h = self.rrc(self.regs.h),
            0x0D => self.regs.l = self.rrc(self.regs.l),
            0x0E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.rrc(dat);
                self.write8(addr, ret);
            }
            0x0F => self.regs.a = self.rrc(self.regs.a),

            // RL
            0x10 => self.regs.b = self.rl(self.regs.b),
            0x11 => self.regs.c = self.rl(self.regs.c),
            0x12 => self.regs.d = self.rl(self.regs.d),
            0x13 => self.regs.e = self.rl(self.regs.e),
            0x14 => self.regs.h = self.rl(self.regs.h),
            0x15 => self.regs.l = self.rl(self.regs.l),
            0x16 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.rl(dat);
                self.write8(addr, ret);
            }
            0x17 => self.regs.a = self.rl(self.regs.a),

            // RR
            0x18 => self.regs.b = self.rr(self.regs.b),
            0x19 => self.regs.c = self.rr(self.regs.c),
            0x1A => self.regs.d = self.rr(self.regs.d),
            0x1B => self.regs.e = self.rr(self.regs.e),
            0x1C => self.regs.h = self.rr(self.regs.h),
            0x1D => self.regs.l = self.rr(self.regs.l),
            0x1E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.rr(dat);
                self.write8(addr, ret);
            }
            0x1F => self.regs.a = self.rr(self.regs.a),

            // SLA
            0x20 => self.regs.b = self.sla(self.regs.b),
            0x21 => self.regs.c = self.sla(self.regs.c),
            0x22 => self.regs.d = self.sla(self.regs.d),
            0x23 => self.regs.e = self.sla(self.regs.e),
            0x24 => self.regs.h = self.sla(self.regs.h),
            0x25 => self.regs.l = self.sla(self.regs.l),
            0x26 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.sla(dat);
                self.write8(addr, ret);
            }
            0x27 => self.regs.a = self.sla(self.regs.a),

            // SRA
            0x28 => self.regs.b = self.sra(self.regs.b),
            0x29 => self.regs.c = self.sra(self.regs.c),
            0x2A => self.regs.d = self.sra(self.regs.d),
            0x2B => self.regs.e = self.sra(self.regs.e),
            0x2C => self.regs.h = self.sra(self.regs.h),
            0x2D => self.regs.l = self.sra(self.regs.l),
            0x2E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.sra(dat);
                self.write8(addr, ret);
            }
            0x2F => self.regs.a = self.sra(self.regs.a),

            // SWAP
            0x30 => self.regs.b = self.swap(self.regs.b),
            0x31 => self.regs.c = self.swap(self.regs.c),
            0x32 => self.regs.d = self.swap(self.regs.d),
            0x33 => self.regs.e = self.swap(self.regs.e),
            0x34 => self.regs.h = self.swap(self.regs.h),
            0x35 => self.regs.l = self.swap(self.regs.l),
            0x36 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.swap(dat);
                self.write8(addr, ret);
            }
            0x37 => self.regs.a = self.swap(self.regs.a),

            // SRL
            0x38 => self.regs.b = self.srl(self.regs.b),
            0x39 => self.regs.c = self.srl(self.regs.c),
            0x3A => self.regs.d = self.srl(self.regs.d),
            0x3B => self.regs.e = self.srl(self.regs.e),
            0x3C => self.regs.h = self.srl(self.regs.h),
            0x3D => self.regs.l = self.srl(self.regs.l),
            0x3E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                let ret = self.srl(dat);
                self.write8(addr, ret);
            }
            0x3F => self.regs.a = self.srl(self.regs.a),

            // BIT 0 
            0x40 => self.bit(self.regs.b, 0),
            0x41 => self.bit(self.regs.c, 0),
            0x42 => self.bit(self.regs.d, 0),
            0x43 => self.bit(self.regs.e, 0),
            0x44 => self.bit(self.regs.h, 0),
            0x45 => self.bit(self.regs.l, 0),
            0x46 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 0);
            }
            0x47 => self.bit(self.regs.a, 0),

            // BIT 1
            0x48 => self.bit(self.regs.b, 1),
            0x49 => self.bit(self.regs.c, 1),
            0x4A => self.bit(self.regs.d, 1),
            0x4B => self.bit(self.regs.e, 1),
            0x4C => self.bit(self.regs.h, 1),
            0x4D => self.bit(self.regs.l, 1),
            0x4E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 1);
            }
            0x4F => self.bit(self.regs.a, 1),

            // BIT 2
            0x50 => self.bit(self.regs.b, 2),
            0x51 => self.bit(self.regs.c, 2),
            0x52 => self.bit(self.regs.d, 2),
            0x53 => self.bit(self.regs.e, 2),
            0x54 => self.bit(self.regs.h, 2),
            0x55 => self.bit(self.regs.l, 2),
            0x56 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 2);
            }
            0x57 => self.bit(self.regs.a, 2),

            // BIT 3
            0x58 => self.bit(self.regs.b, 3),
            0x59 => self.bit(self.regs.c, 3),
            0x5A => self.bit(self.regs.d, 3),
            0x5B => self.bit(self.regs.e, 3),
            0x5C => self.bit(self.regs.h, 3),
            0x5D => self.bit(self.regs.l, 3),
            0x5E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 3);
            }
            0x5F => self.bit(self.regs.a, 3),

            // BIT 4
            0x60 => self.bit(self.regs.b, 4),
            0x61 => self.bit(self.regs.c, 4),
            0x62 => self.bit(self.regs.d, 4),
            0x63 => self.bit(self.regs.e, 4),
            0x64 => self.bit(self.regs.h, 4),
            0x65 => self.bit(self.regs.l, 4),
            0x66 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 4);
            }
            0x67 => self.bit(self.regs.a, 4),

            // BIT 5
            0x68 => self.bit(self.regs.b, 5),
            0x69 => self.bit(self.regs.c, 5),
            0x6A => self.bit(self.regs.d, 5),
            0x6B => self.bit(self.regs.e, 5),
            0x6C => self.bit(self.regs.h, 5),
            0x6D => self.bit(self.regs.l, 5),
            0x6E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 5);
            }
            0x6F => self.bit(self.regs.a, 5),

            // BIT 6
            0x70 => self.bit(self.regs.b, 6),
            0x71 => self.bit(self.regs.c, 6),
            0x72 => self.bit(self.regs.d, 6),
            0x73 => self.bit(self.regs.e, 6),
            0x74 => self.bit(self.regs.h, 6),
            0x75 => self.bit(self.regs.l, 6),
            0x76 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 6);
            }
            0x77 => self.bit(self.regs.a, 6),

            // BIT 7
            0x78 => self.bit(self.regs.b, 7),
            0x79 => self.bit(self.regs.c, 7),
            0x7A => self.bit(self.regs.d, 7),
            0x7B => self.bit(self.regs.e, 7),
            0x7C => self.bit(self.regs.h, 7),
            0x7D => self.bit(self.regs.l, 7),
            0x7E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.bit(dat, 7);
            }
            0x7F => self.bit(self.regs.a, 7),

            // RES 0
            0x80 => self.regs.b &= !(0x01),
            0x81 => self.regs.c &= !(0x01),
            0x82 => self.regs.d &= !(0x01),
            0x83 => self.regs.e &= !(0x01),
            0x84 => self.regs.h &= !(0x01),
            0x85 => self.regs.l &= !(0x01),
            0x86 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01));
            }
            0x87 => self.regs.a &= !(0x01),

            // RES 1
            0x88 => self.regs.b &= !(0x01 << 1),
            0x89 => self.regs.c &= !(0x01 << 1),
            0x8A => self.regs.d &= !(0x01 << 1),
            0x8B => self.regs.e &= !(0x01 << 1),
            0x8C => self.regs.h &= !(0x01 << 1),
            0x8D => self.regs.l &= !(0x01 << 1),
            0x8E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 1));
            }
            0x8F => self.regs.a &= !(0x01 << 1),

            // RES 2
            0x90 => self.regs.b &= !(0x01 << 2),
            0x91 => self.regs.c &= !(0x01 << 2),
            0x92 => self.regs.d &= !(0x01 << 2),
            0x93 => self.regs.e &= !(0x01 << 2),
            0x94 => self.regs.h &= !(0x01 << 2),
            0x95 => self.regs.l &= !(0x01 << 2),
            0x96 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 2));
            }
            0x97 => self.regs.a &= !(0x01 << 2),

            // RES 3
            0x98 => self.regs.b &= !(0x01 << 3),
            0x99 => self.regs.c &= !(0x01 << 3),
            0x9A => self.regs.d &= !(0x01 << 3),
            0x9B => self.regs.e &= !(0x01 << 3),
            0x9C => self.regs.h &= !(0x01 << 3),
            0x9D => self.regs.l &= !(0x01 << 3),
            0x9E => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 3));
            }
            0x9F => self.regs.a &= !(0x01 << 3),

            // RES 4
            0xA0 => self.regs.b &= !(0x01 << 4),
            0xA1 => self.regs.c &= !(0x01 << 4),
            0xA2 => self.regs.d &= !(0x01 << 4),
            0xA3 => self.regs.e &= !(0x01 << 4),
            0xA4 => self.regs.h &= !(0x01 << 4),
            0xA5 => self.regs.l &= !(0x01 << 4),
            0xA6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 4));
            }
            0xA7 => self.regs.a &= !(0x01 << 4),

            // RES 5
            0xA8 => self.regs.b &= !(0x01 << 5),
            0xA9 => self.regs.c &= !(0x01 << 5),
            0xAA => self.regs.d &= !(0x01 << 5),
            0xAB => self.regs.e &= !(0x01 << 5),
            0xAC => self.regs.h &= !(0x01 << 5),
            0xAD => self.regs.l &= !(0x01 << 5),
            0xAE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 5));
            }
            0xAF => self.regs.a &= !(0x01 << 5),

            // RES 6
            0xB0 => self.regs.b &= !(0x01 << 6),
            0xB1 => self.regs.c &= !(0x01 << 6),
            0xB2 => self.regs.d &= !(0x01 << 6),
            0xB3 => self.regs.e &= !(0x01 << 6),
            0xB4 => self.regs.h &= !(0x01 << 6),
            0xB5 => self.regs.l &= !(0x01 << 6),
            0xB6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 6));
            }
            0xB7 => self.regs.a &= !(0x01 << 6),

            // RES 7
            0xB8 => self.regs.b &= !(0x01 << 7),
            0xB9 => self.regs.c &= !(0x01 << 7),
            0xBA => self.regs.d &= !(0x01 << 7),
            0xBB => self.regs.e &= !(0x01 << 7),
            0xBC => self.regs.h &= !(0x01 << 7),
            0xBD => self.regs.l &= !(0x01 << 7),
            0xBE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat & !(0x01 << 7));
            }
            0xBF => self.regs.a &= !(0x01 << 7),

            // SET 0
            0xC0 => self.regs.b |= 0x01,
            0xC1 => self.regs.c |= 0x01,
            0xC2 => self.regs.d |= 0x01,
            0xC3 => self.regs.e |= 0x01,
            0xC4 => self.regs.h |= 0x01,
            0xC5 => self.regs.l |= 0x01,
            0xC6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01));
            }
            0xC7 => self.regs.a |= 0x01,

            // SET 1
            0xC8 => self.regs.b |= 0x01 << 1,
            0xC9 => self.regs.c |= 0x01 << 1,
            0xCA => self.regs.d |= 0x01 << 1,
            0xCB => self.regs.e |= 0x01 << 1,
            0xCC => self.regs.h |= 0x01 << 1,
            0xCD => self.regs.l |= 0x01 << 1,
            0xCE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 1));
            }
            0xCF => self.regs.a |= 0x01 << 1,

            // SET 2
            0xD0 => self.regs.b |= 0x01 << 2,
            0xD1 => self.regs.c |= 0x01 << 2,
            0xD2 => self.regs.d |= 0x01 << 2,
            0xD3 => self.regs.e |= 0x01 << 2,
            0xD4 => self.regs.h |= 0x01 << 2,
            0xD5 => self.regs.l |= 0x01 << 2,
            0xD6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 2));
            }
            0xD7 => self.regs.a |= 0x01 << 0,

            // SET 3
            0xD8 => self.regs.b |= 0x01 << 3,
            0xD9 => self.regs.c |= 0x01 << 3,
            0xDA => self.regs.d |= 0x01 << 3,
            0xDB => self.regs.e |= 0x01 << 3,
            0xDC => self.regs.h |= 0x01 << 3,
            0xDD => self.regs.l |= 0x01 << 3,
            0xDE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 3));
            }
            0xDF => self.regs.a |= 0x01 << 3,

            // SET 4
            0xE0 => self.regs.b |= 0x01 << 4,
            0xE1 => self.regs.c |= 0x01 << 4,
            0xE2 => self.regs.d |= 0x01 << 4,
            0xE3 => self.regs.e |= 0x01 << 4,
            0xE4 => self.regs.h |= 0x01 << 4,
            0xE5 => self.regs.l |= 0x01 << 4,
            0xE6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 4));
            }
            0xE7 => self.regs.a |= 0x01 << 4,

            // SET 5
            0xE8 => self.regs.b |= 0x01 << 5,
            0xE9 => self.regs.c |= 0x01 << 5,
            0xEA => self.regs.d |= 0x01 << 5,
            0xEB => self.regs.e |= 0x01 << 5,
            0xEC => self.regs.h |= 0x01 << 5,
            0xED => self.regs.l |= 0x01 << 5,
            0xEE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 5));
            }
            0xEF => self.regs.a |= 0x01 << 5,

            // SET 6
            0xF0 => self.regs.b |= 0x01 << 6,
            0xF1 => self.regs.c |= 0x01 << 6,
            0xF2 => self.regs.d |= 0x01 << 6,
            0xF3 => self.regs.e |= 0x01 << 6,
            0xF4 => self.regs.h |= 0x01 << 6,
            0xF5 => self.regs.l |= 0x01 << 6,
            0xF6 => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 6));
            }
            0xF7 => self.regs.a |= 0x01 << 6,

            // SET 7
            0xF8 => self.regs.b |= 0x01 << 7,
            0xF9 => self.regs.c |= 0x01 << 7,
            0xFA => self.regs.d |= 0x01 << 7,
            0xFB => self.regs.e |= 0x01 << 7,
            0xFC => self.regs.h |= 0x01 << 7,
            0xFD => self.regs.l |= 0x01 << 7,
            0xFE => {
                let addr = self.regs.get_hl();
                let dat = self.read8(addr);
                self.write8(addr, dat | (0x01 << 7));
            }
            0xFF => self.regs.a |= 0x01 << 7,
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        self.mmc.borrow_mut().read(addr)
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        //println!("read8 addr:{:x}, dat:{:x}", addr, self.read8(addr));
        //println!("read8 addr:{:x}, dat:{:x}", addr+1, self.read8(addr+1));
        u16::from(self.read8(addr)) | u16::from(self.read8(addr.wrapping_add(1))) << 8
    }

    pub fn imm8(&mut self) -> u8 {
        let ret = self.read8(self.regs.pc);
        self.regs.pc += 1;
        ret
    }

    pub fn imm16(&mut self) -> u16 {
        let ret = self.read16(self.regs.pc);
        // println!("imm16 pc:{:x} ret:{:x}", self.regs.pc, ret);
        self.regs.pc += 2;
        ret
    }

    pub fn write8(&mut self, addr: u16, dat: u8) {
        self.mmc.borrow_mut().write(addr, dat);
    }

    pub fn write16(&mut self, addr: u16, dat: u16) {
        //println!("write8 addr:{:x}, dat:{:x}", addr, dat&0xff);
        //println!("write8 addr:{:x}, dat:{:x}", addr+1, dat>>8);
        self.write8(addr.wrapping_add(1), (dat >> 8) as u8);
        self.write8(addr, (dat & 0xFF) as u8);
    }

    pub fn inc(&mut self, r: u8) -> u8 {
        let ret = r.wrapping_add(1);
        let half = (r & 0x0F) + 1;
        
        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(half & 0x10 > 0);

        ret
    }

    pub fn dec(&mut self, r: u8) -> u8 {
        let ret = r.wrapping_sub(1);
        
        self.regs.set_z(ret == 0);
        self.regs.set_n(true);
        self.regs.set_h(r & 0x0F < 1);

        ret
    }

    pub fn add(&mut self, r: u8) -> u8 {
        let ret = self.regs.a.wrapping_add(r);
        let half_carry = (self.regs.a & 0xF) + (r & 0xF) > 0xF;
        let carry = u16::from(self.regs.a) + u16::from(r) > 0xFF;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);

        ret
    }

    pub fn add_hl(&mut self, r: u16) {
        let hl = self.regs.get_hl();
        let ret = hl.wrapping_add(r);
        let half_carry = (hl & 0xFFF) + (r & 0xFFF) > 0xFFF;
        let carry = u32::from(hl) + u32::from(r) > 0xFFFF;

        self.regs.set_n(false);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);

        self.regs.set_hl(ret);
    }

    pub fn adc(&mut self, r: u8) -> u8 {
        let c = self.regs.get_c() as u8;
        let ret = self.regs.a.wrapping_add(r).wrapping_add(c);
        let half_carry = (self.regs.a & 0xF) + (r & 0xF) + (c & 0xF) > 0xF;
        let carry = u16::from(self.regs.a) + u16::from(r) + u16::from(c) > 0xFF;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);

        ret
    }

    pub fn sub(&mut self, r: u8) -> u8 {
        let ret = self.regs.a.wrapping_sub(r);
        let half_carry = (self.regs.a & 0xF) < (r & 0xF);
        let carry = self.regs.a < r;

        self.regs.set_z(ret == 0);
        self.regs.set_n(true);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);

        ret
    }

    pub fn sbc(&mut self, r: u8) -> u8 {
        let c = self.regs.get_c() as u8;
        let ret = self.regs.a.wrapping_sub(r).wrapping_sub(c);
        let half_carry = (self.regs.a & 0xF) < (r & 0xF) + (c & 0xF);
        let carry = u16::from(self.regs.a) < u16::from(r) + u16::from(c);

        self.regs.set_z(ret == 0);
        self.regs.set_n(true);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);

        ret
    }

    pub fn and(&mut self, r: u8) -> u8 {
        let ret = self.regs.a & r;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(true);
        self.regs.set_c(false);

        ret
    }

    pub fn xor(&mut self, r: u8) -> u8 {
        let ret = self.regs.a ^ r;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        ret
    }

    pub fn or(&mut self, r: u8) -> u8 {
        let ret = self.regs.a | r;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        ret
    }

    pub fn cp(&mut self, r: u8) {
        let half_carry = (self.regs.a & 0xF) < (r & 0xF);
        let carry = self.regs.a < r;

        self.regs.set_z(self.regs.a == r);
        self.regs.set_n(true);
        self.regs.set_h(half_carry);
        self.regs.set_c(carry);
    }

    pub fn pop(&mut self) -> u16 {
        let ret = self.read16(self.regs.sp);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        ret
    }

    pub fn push(&mut self, dat: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        self.write16(self.regs.sp, dat);
    }

    pub fn jr(&mut self, offset: u8) {
        self.regs.pc = ((u32::from(self.regs.pc) as i32) + i32::from(offset as i8)) as u16;
    }

    pub fn rl(&mut self, r: u8) -> u8 {
        let carry = r & 0x80 > 0;
        let mut ret = r << 1;
        if self.regs.get_c() { ret |= 0x01 };

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn rlc(&mut self, r: u8) -> u8 {
        let carry = r & 0x80 > 0;
        let mut ret = r << 1;
        if carry { ret |= 0x01 };

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn rr(&mut self, r: u8) -> u8 {
        let carry = r & 0x01 > 0;
        let mut ret = r >> 1;
        if self.regs.get_c() { ret |= 0x80 };

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn rrc(&mut self, r: u8) -> u8 {
        let carry = r & 0x01 > 0;
        let mut ret = r >> 1;
        if carry { ret |= 0x80 };

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn sla(&mut self, r: u8) -> u8 {
        let carry = r & 0x80 > 0;
        let ret = r << 1;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn sra(&mut self, r: u8) -> u8 {
        let carry = r & 0x01 > 0;
        let ret = (r & 0x80) | (r >> 1);

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn swap(&mut self, r: u8) -> u8 {
        let ret = (r << 4) | (r >> 4);

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(false);

        ret
    }

    pub fn srl(&mut self, r: u8) -> u8 {
        let carry = r & 0x01 > 0;
        let ret = r >> 1;

        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(false);
        self.regs.set_c(carry);

        ret
    }

    pub fn bit(&mut self, r: u8, n: u8) {
        let bit = r & (0x01 << n);
        self.regs.set_z(bit == 0);
        self.regs.set_n(false);
        self.regs.set_h(true);
    }

    pub fn handle_interrupt(&mut self) -> u32 {
        if !self.ime && !self.halt {
            return 0;
        }

        let int_enable: u8 = self.read8(IoRegs::IE as u16);
        let mut int_flag: u8 = self.read8(IoRegs::IF as u16);
        println!("cpu hi inte:{:0>8b}", int_enable);
        println!("cpu hi intf:{:0>8b}", int_flag);

        let fired_interrupt: u8 = int_enable & int_flag;
        if fired_interrupt == 0 {
            return 0;
        }
        self.halt = false;

        if !self.ime {
            return 0;
        }
        self.ime = false;

        /*
        let n = fired_interrupt.trailing_zeros();
        int_flag = int_flag & !(1 << n);
        self.write8(IoRegs::IF as u16, int_flag);
        */

        self.push(self.regs.pc);
        if fired_interrupt > 0 {
            if fired_interrupt & (IntFlag::VBLANK as u8) > 0 {
                self.regs.pc = 0x40;
                int_flag &= !(IntFlag::VBLANK as u8);
            } else if fired_interrupt & (IntFlag::STAT as u8) > 0 {
                self.regs.pc = 0x48;
                int_flag &= !(IntFlag::STAT as u8);
            } else if fired_interrupt & (IntFlag::TIMER as u8) > 0 {
                self.regs.pc = 0x58;
                int_flag &= !(IntFlag::SERIAL as u8);
            } else if fired_interrupt & (IntFlag::JOYPAD as u8) > 0 {
                self.regs.pc = 0x60;
                int_flag &= !(IntFlag::JOYPAD as u8);
            }
        }
        self.write8(IoRegs::IF as u16, int_flag);
        // self.regs.pc = 0x0040 | ((n as u16) << 3);

        4
    }

    pub fn debug_out(&mut self) {
        let opcode = self.read8(self.regs.pc);

        let inst_name: [&str; 0x100] = [
            "NOP", "LD BC,nn", "LD (BC),A", "INC BC", "INC B", "DEC B", "LD B,n", "RLCA",
            "LD (nn),SP", "ADD HL,BC", "LD A,(BC)", "DEC BC", "INC C", "DEC C", "LD C,n", "RRCA",
            "STOP", "LD DE,nn", "LD (DE),A", "INC DE", "INC D", "DEC D", "LD D,n", "RLA",
            "JR n", "ADD HL,DE", "LD A,(DE)", "DEC DE", "INC E", "DEC E", "LD E,n", "RRA",
            "JR NZ,n", "LD HL,nn", "LD (HL+),A", "INC HL", "INC H", "DEC H", "LD H,n", "DAA",
            "JR Z,n", "ADD HL,HL", "LD A,(HLI)", "DEC HL", "INC L", "DEC L", "LD L,n", "CPL",
            "JR NC,n", "LD SP,nn", "LD (HL-),A", "INC SP", "INC (HL)", "DEC (HL)", "LD (HL),n", "SCF",
            "JR C,n", "ADD HL,SP", "LD A,(HLD)", "DEC SP", "INC A", "DEC A", "LDA,n", "CCF",
            "LD B,B", "LD B,C", "LD B,D", "LD B,E", "LD B,H", "LD B,L", "LD B,(HL)", "LD B,A",
            "LD C,B", "LD C,C", "LD C,D", "LD C,E", "LD C,H", "LD C,L", "LD C,(HL)", "LD C,A",
            "LD D,B", "LD D,C", "LD D,D", "LD D,E", "LD D,H", "LD D,L", "LD D,(HL)", "LD D,A",
            "LD E,B", "LD E,C", "LD E,D", "LD E,E", "LD E,H", "LD E,L", "LD E,(HL)", "LD E,A",
            "LD H,B", "LD H,C", "LD H,D", "LD H,E", "LD H,H", "LD H,L", "LD H,(HL)", "LD H,A",
            "LD L,B", "LD L,C", "LD L,D", "LD L,E", "LD L,H", "LD L,L", "LD L,(HL)", "LD L,A",
            "LD (HL),B", "LD (HL),C", "LD (HL),D", "LD (HL),E", "LD (HL),H", "LD (HL),L", "HALT", "LD (HL),A",
            "LD A,B", "LD A,C", "LD A,D", "LD A,E", "LD A,H", "LD A,L", "LD A,(HL)", "LD A,A",
            "ADD A,B", "ADD A,C", "ADD A,D", "ADD A,E", "ADD A,H", "ADD A,L", "ADD A,(HL)", "ADD A,A",
            "ADC A,B", "ADC A,C", "ADC A,D", "ADC A,E", "ADC A,H", "ADC A,L", "ADC A,(HL)", "ADC A,A",
            "SUB B", "SUB C", "SUB D", "SUB E", "SUB H", "SUB L", "SUB (HL)", "SUB A",
            "SBC A,B", "SBC A,C", "SBC A,D", "SBC A,E", "SBC A,H", "SBC A,L", "SBC A,(HL)", "SBC A,A",
            "AND B", "AND C", "AND D", "AND E", "AND H", "AND L", "AND (HL)", "AND A",
            "XOR B", "XOR C", "XOR D", "XOR E", "XOR H", "XOR L", "XOR (HL)", "XOR A",
            "OR B", "OR C", "OR D", "OR E", "OR H", "OR L", "OR (HL)", "OR A",
            "CP B", "CP C", "CP D", "CP E", "CP H", "CP L", "CP (HL)", "CP A",
            "RET NZ", "POP BC", "JP NZ,nn", "JP nn", "CALL NZ,nn", "PUSH BC", "ADD A,n", "RST ",
            "RET Z", "RET", "JP Z,nn", "cb opcode", "CALL Z,nn", "CALL nn", "ADC A,n", "RST 0x08",
            "RET NC", "POP DE", "JP NC,nn", "unused opcode", "CALL NC,nn", "PUSH DE", "SUB n", "RST 0x10",
            "RET C", "RETI", "JP C,nn", "unused opcode", "CALL C,nn", "unused opcode", "SBC A,n", "RST 0x18",
            "LD (0xFF00+n),A", "POP HL", "LD (0xFF00+C),A", "unused opcode", "unused opcode", "PUSH HL", "AND n", "RST 0x20",
            "ADD SP,n", "JP (HL)", "LD (nn),A", "unused opcode", "unused opcode", "unused opcode", "XOR n", "RST 0x28",
            "LD A,(0xFF00+n)", "POP AF", "LD A,(0xFF00+C)", "DI", "unused opcode", "PUSH AF", "OR n", "RST 0x30",
            "LD HL,SP", "LD SP,HL", "LD A,(nn)", "EI", "unused opcode", "unused opcode", "CP n", "RST 0x38",
        ];

        let cb_inst_name: [&str; 0x100] = [
            "RLC B", "RLC C", "RLC D", "RLC E", "RLC H", "RLC L", "RLC (HL)",
            "RLC A", "RRC B", "RRC C", "RRC D", "RRC E", "RRC H", "RRC L", "RRC (HL)", "RRC A",
            "RL B", "RL C", "RL D", "RL E", "RL H", "RL L ", "RL (HL)", "RL A",
            "RR B", "RR C", "RR D", "RR E", "RR H", "RR L", "RR (HL)", "RR A",
            "SLA B", "SLA C", "SLA D", "SLA E", "SLA H", "SLA L", "SLA (HL)",
            "SLA A", "SRA B", "SRA C", "SRA D", "SRA E", "SRA H", "SRA L", "SRA (HL)", "SRA A",
            "SWAP B", "SWAP C", "SWAP D", "SWAP E", "SWAP H", "SWAP L", "SWAP (HL)", "SWAP A",
            "SRL B", "SRL C", "SRL D", "SRL E", "SRL H", "SRL L", "SRL (HL)", "SRL A",
            "BIT 0 B", "BIT 0 C", "BIT 0 D", "BIT 0 E", "BIT 0 H", "BIT 0 L", "BIT 0 (HL)", "BIT 0 A",
            "BIT 1 B", "BIT 1 C", "BIT 1 D", "BIT 1 E", "BIT 1 H", "BIT 1 L", "BIT 1 (HL)", "BIT 1 A",
            "BIT 2 B", "BIT 2 C", "BIT 2 D", "BIT 2 E", "BIT 2 H", "BIT 2 L", "BIT 2 (HL)", "BIT 2 A",
            "BIT 3 B", "BIT 3 C", "BIT 3 D", "BIT 3 E", "BIT 3 H", "BIT 3 L", "BIT 3 (HL)", "BIT 3 A",
            "BIT 4 B", "BIT 4 C", "BIT 4 D", "BIT 4 E", "BIT 4 H", "BIT 4 L", "BIT 4 (HL)", "BIT 4 A",
            "BIT 5 B", "BIT 5 C", "BIT 5 D", "BIT 5 E", "BIT 5 H", "BIT 5 L", "BIT 5 (HL)", "BIT 5 A",
            "BIT 6 B", "BIT 6 C", "BIT 6 D", "BIT 6 E", "BIT 6 H", "BIT 6 L", "BIT 6 (HL)", "BIT 6 A",
            "BIT 7 B", "BIT 7 C", "BIT 7 D", "BIT 7 E", "BIT 7 H", "BIT 7 L", "BIT 7 (HL)", "BIT 7 A",
            "RES 0 B", "RES 0 C", "RES 0 D", "RES 0 E", "RES 0 H", "RES 0 L", "RES 0 (HL)", "RES 0 A",
            "RES 1 B", "RES 1 C", "RES 1 D", "RES 1 E", "RES 1 H", "RES 1 L", "RES 1 (HL)", "RES 1 A",
            "RES 2 B", "RES 2 C", "RES 2 D", "RES 2 E", "RES 2 H", "RES 2 L", "RES 2 (HL)", "RES 2 A",
            "RES 3 B", "RES 3 C", "RES 3 D", "RES 3 E", "RES 3 H", "RES 3 L", "RES 3 (HL)", "RES 3 A",
            "RES 4 B", "RES 4 C", "RES 4 D", "RES 4 E", "RES 4 H", "RES 4 L", "RES 4 (HL)", "RES 4 A", "RES 5 B",
            "RES 5 C", "RES 5 D", "RES 5 E", "RES 5 H", "RES 5 L", "RES 5 (HL)", "RES 5 A",
            "RES 6 B", "RES 6 C", "RES 6 D", "RES 6 E", "RES 6 H", "RES 6 L", "RES 6 (HL)", "RES 6 A", "RES 7 B",
            "RES 7 C", "RES 7 D", "RES 7 E", "RES 7 H", "RES 7 L", "RES 7 (HL)", "RES 7 A",
            "SET 0 B", "SET 0 C", "SET 0 D", "SET 0 E", "SET 0 H", "SET 0 L", "SET 0 (HL)", "SET 0 A", "SET 1 B",
            "SET 1 C", "SET 1 D", "SET 1 E", "SET 1 H", "SET 1 L", "SET 1 (HL)", "SET 1 A",
            "SET 2 B", "SET 2 C", "SET 2 D", "SET 2 E", "SET 2 H", "SET 2 L", "SET 2 (HL)", "SET 2 A",
            "SET 3 B", "SET 3 C", "SET 3 D", "SET 3 E", "SET 3 H", "SET 3 L", "SET 3 (HL)", "SET 3 A",
            "SET 4 B", "SET 4 C", "SET 4 D", "SET 4 E", "SET 4 H", "SET 4 L", "SET 4 (HL)", "SET 4 A",
            "SET 5 B", "SET 5 C", "SET 5 D", "SET 5 E", "SET 5 H", "SET 5 L", "SET 5 (HL)", "SET 5 A",
            "SET 6 B", "SET 6 C", "SET 6 D", "SET 6 E", "SET 6 H", "SET 6 L", "SET 6 (HL)", "SET 6 A",
            "SET 7 B", "SET 7 C", "SET 7 D", "SET 7 E", "SET 7 H", "SET 7 L", "SET 7 (HL)", "SET 7 A",
        ];

        let mut str = inst_name[opcode as usize];
        let mut str_opcode: u16 = opcode as u16;
        if opcode == 0xCB {
            let cb_opcode = self.read8(self.regs.pc + 1);
            str = cb_inst_name[cb_opcode as usize];
            str_opcode = u16::from(opcode) << 8 | u16::from(cb_opcode);
        }

        println!("PC:{:>04x} SP:{:>04x}, A:{:>02x} F:{:>02x} B:{:>02x} C:{:>02x} D:{:>02x} E:{:>02x} H:{:>02x} L:{:>02x}, 0x{:>02x} {}",
                self.regs.pc, self.regs.sp, self.regs.a, self.regs.f, self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l, str_opcode, str);
    }
}