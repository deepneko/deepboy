use std::{cell::RefCell, rc::Rc};

use crate::register::ByteRegister;

pub struct Joypad {
    int_flag: Rc<RefCell<ByteRegister>>,
    select_switch: ByteRegister,
    right: bool,
    left: bool,
    up: bool,
    down: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool,
}

impl Joypad {
    pub fn new(int_flag: Rc<RefCell<ByteRegister>>) -> Self {
        Joypad {
            int_flag: int_flag,
            select_switch: ByteRegister::new(),
            right: false,
            left: false,
            up: false,
            down: false,
            a: false,
            b: false,
            select: false,
            start: false,
        }
    }

    pub fn key_down(&mut self, key: minifb::Key) {
        self.int_flag.borrow_mut().set_bit(4, true);
        match key {
            minifb::Key::Right => { self.right = true },
            minifb::Key::Left => { self.left = true },
            minifb::Key::Up => { self.up = true },
            minifb::Key::Down => { self.down = true },
            minifb::Key::A => { self.a = true },
            minifb::Key::B => { self.b = true },
            minifb::Key::Space => { self.select = true },
            minifb::Key::Enter => { self.start = true },
            _ => {}
        } 
    }

    pub fn key_up(&mut self, key: minifb::Key) {
        match key {
            minifb::Key::Right => { self.right = false },
            minifb::Key::Left => { self.left = false },
            minifb::Key::Up => { self.up = false },
            minifb::Key::Down => { self.down = false },
            minifb::Key::A => { self.a = false },
            minifb::Key::B => { self.b = false },
            minifb::Key::Space => { self.select = false },
            minifb::Key::Enter => { self.start = false },
            _ => {}
        } 
    }

    pub fn read(&self, addr: u16) -> u8 {
        assert_eq!(addr, 0xff00);
        let mut keys = ByteRegister::new();
        keys.set(0b1111);

        if !self.select_switch.check_bit(4) {
            keys.set_bit(0, !self.right);
            keys.set_bit(1, !self.left);
            keys.set_bit(2, !self.up);
            keys.set_bit(3, !self.down);
            return self.select_switch.data | keys.get();
        }

        if !self.select_switch.check_bit(5) {
            keys.set_bit(0, !self.a);
            keys.set_bit(1, !self.b);
            keys.set_bit(2, !self.select);
            keys.set_bit(3, !self.start);
            return self.select_switch.data | keys.get();
        }

        return self.select_switch.data;
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        assert_eq!(addr, 0xff00);
        self.select_switch.set(dat);
    }
}
