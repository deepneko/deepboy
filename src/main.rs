#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = &args[1];
    println!("rom: {rom}", rom=fname);

    let gameboy: &mut Gameboy;
    Gameboy::load_rom(gameboy, fname);
}
