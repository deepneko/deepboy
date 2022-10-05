#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    // let bootstrap_name = &args[2];
    // println!("boot_strap: {boot}", boot=bootstrap_name);

    let mut gameboy = Gameboy::new();
    let debug = true;
    gameboy.cpu.set_debug();
    // gameboy.mmc.borrow_mut().ppu.set_debug();

    gameboy.load_rom(rom_name);
    // gameboy.load_bootstrap(bootstrap_name);

    let mut count: u32 = 0;
    loop {
        if debug {
            println!("count:{}", count);
            if count == 5000000 {
                panic!("For debug.")
            }
            count = count + 1;
        }
        gameboy.exec_frame();
    }
}
