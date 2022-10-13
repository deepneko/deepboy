use super::Mapper;

pub struct NoMbc {
    ram: Vec<u8>,
}

impl NoMbc {
    pub fn new(ram: Vec<u8>) -> Self {
        NoMbc {
            ram: ram,
        }
    }
}

impl Mapper for NoMbc {
    fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn write(&mut self, _: u16, _: u8) {}
}