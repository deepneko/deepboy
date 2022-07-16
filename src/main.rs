#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let mut gameboy = Gameboy::new();
    gameboy.load(rom_name);
}
