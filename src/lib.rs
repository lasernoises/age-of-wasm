#[cfg(feature = "buddy-alloc")]
mod alloc;
mod sprites;
mod wasm4;
mod game;

use core::mem::MaybeUninit;
use wasm4::*;
use game::*;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: &'static [u8],
}

pub fn set_draw_colors(colors: u16) {
    unsafe {
        *DRAW_COLORS = colors;
    }
}

static mut STATE: MaybeUninit<Game> = MaybeUninit::uninit();

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [
            // 0x000000,
            0x0077ff,
            0x444444,
            0x888888,
            0xFF0000,
        ];

        STATE.write(Game::new());
    }
}

#[no_mangle]
fn update() {
    unsafe {
        STATE.assume_init_mut().update();
    }
}
