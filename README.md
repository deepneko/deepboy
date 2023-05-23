# DEEPBOY 
Simple Game Boy emulator in Rust.

No audio function is supported currently.

## Usage
You can start a game with the command.
```s
$ cargo run "./roms/zelda.gb"
```

## Key bindings
| Gameboy       | Keyboard  |
|-----------	|----------	|
| Up        	| Up       	|
| Down      	| Down     	|
| Left      	| Left     	|
| Right     	| Right    	|
| A Button      | A      	|
| B Button      | B         |
| Start    	    | Enter    	|
| Select     	| Space   	|

## Games
Confirmed these games worked well.

### No MBC
<img src="https://github.com/deepneko/deepboy/blob/images/tetris.png" alt="Tetris" width="150"/> <img src="https://github.com/deepneko/deepboy/blob/images/super_mario.png" alt="Super Mario" width="150"/>

### MBC1
<img src="https://github.com/deepneko/deepboy/blob/images/zelda.png" alt="Zelda" width="150"/> <img src="https://github.com/deepneko/deepboy/blob/images/rockman.png" alt="Rockman World" width="150"/> <img src="https://github.com/deepneko/deepboy/blob/images/rockman2.png" alt="Rockman World2" width="150"/> <img src="https://github.com/deepneko/deepboy/blob/images/metroid2.png" alt="Metroid2" width="150"/> <img src="https://github.com/deepneko/deepboy/blob/images/pokemon_midori.png" alt="Pokemon Midori" width="150"/>

## Reference
https://imrannazar.com/GameBoy-Emulation-in-JavaScript
https://gbdev.io/pandocs/

## License
MIT License. Please see LICENSE file.
