use crate::sprites::*;

type Animation = &'static [&'static [u8]];

struct UnitType {
    width: u32,
    height: u32,

    standing: Animation,
    walking: Animation,
    attack: Animation,

    // frames per pixel
    speed: u8,

    health: u8,
    range: u8,
    damage: u8,
}


const BOXER: UnitType = UnitType {
    width: UNIT4_WIDTH,
    height: UNIT4_HEIGHT,

    standing: &[
        UNIT4,
    ],
    walking: &[
        UNIT4,
        UNIT5,
    ],
    attack: &[
        UNIT4_ATTACK1,
        UNIT4_ATTACK2,
    ],

    speed: 4,
    health: 16,
    range: 16,
    damage: 4,
};
