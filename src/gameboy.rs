use super::rom::Rom;
use super::cpu::CPU;
use super::mmc::MMC;
use super::output::Output;

pub struct Gameboy {
    pub rom: Rom,
    pub cpu: CPU,
    pub mmc: MMC,
    pub output: Output,
}

impl Gameboy {
    pub fn new(output: Output) -> Self {
        Gameboy {
            rom: Rom::new(),
            cpu: CPU::new(),
            mmc: MMC::new(),
            output: output,
        }
    }

    pub fn load_rom(&mut self, fname: &String) {
        self.rom.load(fname);
    }

    pub fn load_bootstrap(&mut self, fname: &String) {
        self.rom.load_bootstrap(fname);
    }

    pub fn exec_frame(&mut self) {
        self.cpu.run(0);
        self.output.write_screen();
    }
}