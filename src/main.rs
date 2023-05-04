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

    let mut count: u32 = 0;
    loop {
        if debug {
            // println!("count:{}", count);
            if count == 300000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 400000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 600000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 800000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }

            if count == 26000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Right);
            }
            if count == 28000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::A);
            }
            if count == 28100000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::A);
            }
            if count == 29000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::B);
            }
            if count == 29100000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::B);
            }
            if count == 30000000 {
                panic!("For debug.")
            }
            count = count + 1;
        }
        gameboy.exec_frame();
    }
}
