use crate::register::ByteRegister;

pub struct Joypad {
    direction_select: bool,    
    action_select: bool,
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
    pub fn new() -> Self {
        Joypad {
            direction_select: false,
            action_select: false,
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

    pub fn read(&self, addr: u16) -> u8 {
        let mut buttons = ByteRegister::new();
        buttons.set(0b1111);

        if self.direction_select {
            buttons.set_bit(0, !self.right);
            buttons.set_bit(1, !self.left);
            buttons.set_bit(2, !self.up);
            buttons.set_bit(3, !self.down);
        }

        if self.action_select {
            buttons.set_bit(0, !self.a);
            buttons.set_bit(1, !self.b);
            buttons.set_bit(2, !self.select);
            buttons.set_bit(3, !self.start);
        }

        buttons.set_bit(4, !self.direction_select);
        buttons.set_bit(5, !self.action_select);

        buttons.get()
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.direction_select = (dat >> 4) & 0x1 == 0;
        self.action_select = (dat >> 5) & 0x1 == 0;
    }
}