#![crate_name = "deepboy"]

use std::time::Duration;
use deepboy::gameboy::Gameboy;
use deepboy::output::Output;

Rc<RefCell<Gameboy>>;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let bootstrap_name = &args[2];
    println!("boot_strap: {boot}", boot=bootstrap_name);

    let output = Output::new();
    let mut gameboy = Gameboy::new(output);
    gameboy.load_rom(rom_name);
    gameboy.load_bootstrap(bootstrap_name);

    loop {
        gameboy.exec_frame();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
