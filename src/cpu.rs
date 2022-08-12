use std::{cell::RefCell, rc::Rc};
use crate::mmc::MMC;
use crate::register::*;
use super::register::Register;

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
                self.regs.set_hl(addr + 1);
            }
            0x32 => {
                let addr = self.regs.get_hl();
                self.write8(addr, self.regs.a);
                self.regs.set_hl(addr - 1);
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
            0x25 => self.regs.h = self.inc(self.regs.h),
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
                self.rlc(self.regs.a);
                self.regs.set_z(false);
            }

            // RLA
            0x17 => {
                self.rl(self.regs.a);
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
                self.regs.set_hl(addr + 1);
            }
            0x3A => {
                let addr = self.regs.get_hl();
                self.regs.a = self.read8(addr);
                self.regs.set_hl(addr - 1);
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
                self.rrc(self.regs.a);
                self.regs.set_z(false);
            }

            // RRA
            0x1F => {
                self.rr(self.regs.a);
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
            0xC2 => if !self.regs.get_z() { self.regs.pc = self.imm16() },
            0xC3 => self.regs.pc = self.imm16(),
            0xCA => if self.regs.get_z() { self.regs.pc = self.imm16() },
            0xD2 => if !self.regs.get_c() { self.regs.pc = self.imm16() },
            0xDA => if self.regs.get_c() { self.regs.pc = self.imm16() },
            0xE9 => self.regs.pc = self.regs.get_hl(),  // For real??

            // CALL
            0xC4 => if !self.regs.get_z() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
            }
            0xCC => if self.regs.get_z() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
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
            }
            0xDC => if self.regs.get_c() {
                let dat = self.imm16();
                self.push(self.regs.pc);
                self.regs.pc = dat;
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
                let dat = 0xFF00 | u16::from(self.imm8());
                self.regs.a = self.read8(dat);
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
    }

    pub fn prefix_cb(&mut self) {
        self.opcode = self.imm8();
        self.regs.pc += 1;

        match self.opcode {
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

            // RL
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
        
        self.regs.set_z(ret == 0);
        self.regs.set_n(false);
        self.regs.set_h(half & 0x10 > 0);

        ret
    }

    pub fn dec(&mut self, r: u8) -> u8 {
        let ret = r - 1;
        
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
        let ret = self.regs.a.wrapping_add(r);
        let c = self.regs.get_c() as u8;
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
        let ret = self.regs.a.wrapping_sub(r);
        let c = self.regs.get_c() as u8;
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
        self.regs.sp += 2;
        ret
    }

    pub fn push(&mut self, dat: u16) {
        self.regs.sp -= 2;
        self.write16(self.regs.sp, dat);
    }

    pub fn jr(&mut self, offset: u8) {
        self.regs.pc = (i32::from(self.regs.pc) + i32::from(offset)) as u16;
    }

    pub fn rl(&mut self, r: u8) -> u8 {
        let carry = self.regs.get_c();
        let mut ret = r << 1;
        if carry { ret |= 0x01 };

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
        let carry = self.regs.get_c();
        let mut ret = r >> 1;
        if carry { ret |= 0x80 };

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
}