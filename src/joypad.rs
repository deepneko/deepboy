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

    pub fn key_down(&mut self, key: minifb::Key) {
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

        /*
        println!("joypad read: down:{}", self.down);
        println!("joypad read: start:{}", self.start);
        println!("joypad read: keys:{:8b}", keys.data);
        */
        keys.get()
    }

    pub fn write(&mut self, addr: u16, dat: u8) {
        self.direction_select = (dat >> 4) & 0x1 == 0;
        self.action_select = (dat >> 5) & 0x1 == 0;
    }
}