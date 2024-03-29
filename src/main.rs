#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let mut gameboy = Gameboy::new(rom_name);
    let debug = false;
    // gameboy.cpu.set_debug();
    // gameboy.mmc.borrow_mut().ppu.set_debug();
    // gameboy.mmc.borrow_mut().timer.set_debug();

    let mut count: u32 = 0;
    loop {
        if debug {
            println!("count:{}", count);
            if count == 500000 {
                panic!("For debug.");
            }
            count = count + 1;
        }
        
        if !gameboy.exec_frame() {
            break;
        }
    }
}
