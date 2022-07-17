#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let bootstrap_name = &args[2];
    println!("boot_strap: {boot}", boot=bootstrap_name);

    let mut gameboy = Gameboy::new();
    gameboy.load_rom(rom_name);
    gameboy.load_bootstrap(bootstrap_name)
}
