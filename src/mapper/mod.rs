pub trait Mapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, dat: u8);
}

pub mod mbc1;
pub mod mbc3;