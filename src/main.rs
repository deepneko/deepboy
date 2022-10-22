#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let mut gameboy = Gameboy::new(rom_name);
    let debug = true;
    gameboy.cpu.set_debug();
    // gameboy.mmc.borrow_mut().ppu.set_debug();

    let mut count: u32 = 0;
    loop {
        if debug {
            println!("count:{}", count);
            if count == 500000 {
                gameboy.mmc.borrow_mut().joypad.key_down(0b1000_0000);
            }
            if count == 600000 {
                gameboy.mmc.borrow_mut().joypad.key_up(0b1000_0000);
            }
            if count == 1000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(0b1000_0000);
            }
            if count == 1100000 {
                gameboy.mmc.borrow_mut().joypad.key_up(0b1000_0000);
            }
            if count == 23000000 {
                panic!("For debug.")
            }
            count = count + 1;
        }
        gameboy.exec_frame();
    }
}
