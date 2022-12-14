use crate::wasm4::*;

// unit4
pub const UNIT4_WIDTH: u32 = 12;
pub const UNIT4_HEIGHT: u32 = 16;
pub const UNIT4_FLAGS: u32 = BLIT_2BPP;
pub const UNIT4: &'static [u8] = &[
    0x15, 0x50, 0x00, 0x05, 0x55, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x95, 0x00, 0x00,
    0x15, 0x00, 0x15, 0xa5, 0x94, 0x55, 0xa5, 0x95, 0xa5, 0xa9, 0x96, 0xa0, 0xaa, 0x8a, 0x80, 0xaa,
    0x82, 0x80, 0x55, 0x42, 0x40, 0x55, 0x41, 0x00, 0x51, 0x40, 0x01, 0x51, 0x40, 0x01, 0x41, 0x40,
];

// unit5
pub const UNIT5_WIDTH: u32 = 12;
pub const UNIT5_HEIGHT: u32 = 16;
pub const UNIT5_FLAGS: u32 = BLIT_2BPP;
pub const UNIT5: &'static [u8] = &[
    0x15, 0x50, 0x00, 0x05, 0x55, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x95, 0x00, 0x00,
    0x15, 0x00, 0x05, 0x65, 0x90, 0x15, 0x65, 0x94, 0x2a, 0x69, 0x98, 0x2a, 0xaa, 0xa8, 0x22, 0xaa,
    0x88, 0x21, 0x55, 0x48, 0x11, 0x55, 0x44, 0x01, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x54, 0x00,
];

// unit4_attack1
pub const UNIT4_ATTACK1_WIDTH: u32 = 12;
pub const UNIT4_ATTACK1_HEIGHT: u32 = 16;
pub const UNIT4_ATTACK1_FLAGS: u32 = BLIT_2BPP;
pub const UNIT4_ATTACK1: &'static [u8] = &[
    0x55, 0x40, 0x00, 0x15, 0x54, 0x00, 0x01, 0x6c, 0x00, 0x01, 0x68, 0x00, 0x02, 0x54, 0x00, 0x00,
    0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x69, 0x40, 0x02, 0xa9, 0x40, 0x02, 0xaa, 0x80, 0x02, 0xaa,
    0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00,
];

// unit4_attack2
pub const UNIT4_ATTACK2_WIDTH: u32 = 12;
pub const UNIT4_ATTACK2_HEIGHT: u32 = 16;
pub const UNIT4_ATTACK2_FLAGS: u32 = BLIT_2BPP;
pub const UNIT4_ATTACK2: &'static [u8] = &[
    0x55, 0x40, 0x00, 0x15, 0x54, 0x00, 0x01, 0x6c, 0x00, 0x01, 0x68, 0x00, 0x02, 0x54, 0x00, 0x00,
    0x54, 0x00, 0x01, 0x6a, 0xa5, 0x01, 0x6a, 0xa5, 0x02, 0xa8, 0x00, 0x02, 0xa8, 0x00, 0x02, 0xa8,
    0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00, 0x01, 0x54, 0x00,
];

// unit4_gun1
pub const UNIT4_GUN1_WIDTH: u32 = 12;
pub const UNIT4_GUN1_HEIGHT: u32 = 16;
pub const UNIT4_GUN1_FLAGS: u32 = BLIT_2BPP;
pub const UNIT4_GUN1: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x55, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x95, 0x00, 0x00, 0x15,
    0x00, 0x0c, 0x56, 0xa9, 0x5f, 0x5a, 0xa9, 0x0c, 0xaa, 0x00, 0x00, 0xaa, 0x00, 0x00, 0xaa, 0x00,
    0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00,
];

// unit4_gun2
pub const UNIT4_GUN2_WIDTH: u32 = 12;
pub const UNIT4_GUN2_HEIGHT: u32 = 16;
pub const UNIT4_GUN2_FLAGS: u32 = BLIT_2BPP;
pub const UNIT4_GUN2: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x55, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x95, 0x00, 0x00, 0x15,
    0x00, 0x00, 0x56, 0xa9, 0x50, 0x5a, 0xa9, 0x00, 0xaa, 0x00, 0x00, 0xaa, 0x00, 0x00, 0xaa, 0x00,
    0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00, 0x55, 0x00, 0x00,
];

// base
pub const BASE_WIDTH: u32 = 32;
pub const BASE_HEIGHT: u32 = 32;
pub const BASE_FLAGS: u32 = BLIT_2BPP;
pub const BASE: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x15, 0x55, 0x40, 0x00, 0x00, 0x00, 0x05, 0x55, 0x55, 0x55, 0x55, 0x40, 0x00, 0x00,
    0x55, 0x55, 0x55, 0x55, 0x55, 0x50, 0x00, 0x00, 0x55, 0x55, 0x55, 0x55, 0x55, 0x54, 0x00, 0x00,
    0x55, 0x5a, 0xaa, 0xa5, 0x55, 0x55, 0x00, 0x00, 0x55, 0x6a, 0xaa, 0xaa, 0x55, 0x55, 0x80, 0x00,
    0x55, 0x6a, 0xaa, 0xaa, 0x55, 0x55, 0xa0, 0x00, 0x55, 0xaa, 0xaa, 0xaa, 0x95, 0xaa, 0xa8, 0x00,
    0x55, 0xaa, 0xaa, 0xaa, 0x9a, 0xaa, 0xa8, 0x00, 0x55, 0xaa, 0xaa, 0xaa, 0x9a, 0xaa, 0xaa, 0x00,
    0x6a, 0xa9, 0x55, 0x55, 0x56, 0xaa, 0xaa, 0x00, 0xaa, 0xa5, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x00,
    0xaa, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x00, 0xa9, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x00,
    0xa5, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x80, 0x95, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x80,
    0x55, 0x55, 0x55, 0x55, 0x56, 0xaa, 0xaa, 0x80, 0x55, 0x55, 0x55, 0x55, 0x56, 0xaa, 0xaa, 0x80,
    0x55, 0x55, 0x55, 0x55, 0x56, 0xaa, 0xaa, 0x80, 0x55, 0x55, 0x55, 0x55, 0x56, 0xaa, 0xaa, 0x80,
    0x55, 0x55, 0x55, 0x55, 0x5a, 0xaa, 0xaa, 0x80, 0x55, 0x55, 0x55, 0x55, 0x5a, 0xaa, 0xaa, 0x80,
    0xaa, 0xa5, 0x55, 0x56, 0xaa, 0xa5, 0x6a, 0x80, 0xaa, 0xa9, 0x55, 0x5a, 0xaa, 0xa5, 0x5a, 0x80,
    0xaa, 0xaa, 0x55, 0x6a, 0xaa, 0x95, 0x5a, 0x80, 0xaa, 0xaa, 0x55, 0x6a, 0xaa, 0x95, 0x5a, 0x80,
    0xaa, 0xaa, 0x95, 0x6a, 0xaa, 0xa5, 0x6a, 0x90, 0xaa, 0xaa, 0x95, 0x6a, 0xaa, 0xaa, 0xaa, 0x54,
    0xaa, 0xaa, 0x95, 0x6a, 0x95, 0xaa, 0xa9, 0x54, 0xaa, 0xaa, 0x95, 0x6a, 0x55, 0x6a, 0xa9, 0x54,
];
