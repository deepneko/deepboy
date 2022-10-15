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

    pub fn key_down(&mut self, keys: u8) {
        self.right = keys & 0x1 == 0;
        self.left = (keys >> 1) & 0x1 == 0;
        self.up = (keys >> 2) & 0x1 == 0;
        self.down = (keys >> 3) & 0x1 == 0;
        self.a = (keys >> 4) & 0x1 == 0;
        self.b = (keys >> 5) & 0x1 == 0;
        self.select = (keys >> 6) & 0x1 == 0;
        self.start = (keys >> 7) & 0x1 == 0;
    }

    pub fn key_up(&mut self, keys: u8) {
        self.right = keys & 0x1 != 0;
        self.left = (keys >> 1) & 0x1 != 0;
        self.up = (keys >> 2) & 0x1 != 0;
        self.down = (keys >> 3) & 0x1 != 0;
        self.a = (keys >> 4) & 0x1 != 0;
        self.b = (keys >> 5) & 0x1 != 0;
        self.select = (keys >> 6) & 0x1 != 0;
        self.start = (keys >> 7) & 0x1 != 0;
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mut keys = ByteRegister::new();
        keys.set(0b1111);

        if self.direction_select {
            keys.set_bit(0, !self.right);
            keys.set_bit(1, !self.left);
            keys.set_bit(2, !self.up);
            keys.set_bit(3, !self.down);
        }

        if self.action_select {
            keys.set_bit(0, !self.a);
            keys.set_bit(1, !self.b);
            keys.set_bit(2, !self.select);
            keys.set_bit(3, !self.start);
        }

        keys.set_bit(4, !self.direction_select);
        keys.set_bit(5, !self.action_select);

        keys.get()
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.direction_select = (dat >> 4) & 0x1 == 0;
        self.action_select = (dat >> 5) & 0x1 == 0;
    }
}