use self::{mbc1::Mbc1, mbc3::Mbc3};

pub mod mbc1;
pub mod mbc3;

pub trait Mapper {}

impl Mapper for Mbc1 {}
impl Mapper for Mbc3 {}