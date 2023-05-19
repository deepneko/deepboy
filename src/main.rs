#![crate_name = "deepboy"]

use deepboy::gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];
    println!("rom: {rom}", rom=rom_name);

    let mut gameboy = Gameboy::new(rom_name);
    let debug = true;
    // gameboy.cpu.set_debug();
    // gameboy.mmc.borrow_mut().ppu.set_debug();
    // gameboy.mmc.borrow_mut().timer.set_debug();

    let mut count: u32 = 0;
    loop {
        if debug {
            println!("count:{}", count);
            if count == 900000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 910000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 1500000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 1510000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 2000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 2010000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 2500000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 2510000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 4100000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::A);
            }
            if count == 4110000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::A);
            }
            if count == 5000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 5010000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 6000000 {
                gameboy.mmc.borrow_mut().joypad.key_down(minifb::Key::Enter);
            }
            if count == 6010000 {
                gameboy.mmc.borrow_mut().joypad.key_up(minifb::Key::Enter);
            }
            if count == 7000000 {
                panic!("For debug.");
            }
            count = count + 1;
        }
        
        if !gameboy.exec_frame() {
            break;
        }
    }
}
