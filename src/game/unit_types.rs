use crate::sprites::*;

type Animation = &'static [&'static [u8]];

pub struct UnitType {
    pub width: u32,
    pub height: u32,

    // pub standing: Animation,
    pub walking: Animation,
    pub attack: Animation,

    // frames per pixel
    pub speed: u8,

    pub health: u8,
    pub range: u8,
    pub damage: u8,
}

pub const BOXER: &UnitType = &UnitType {
    width: UNIT4_WIDTH,
    height: UNIT4_HEIGHT,

    // standing: &[
    //     UNIT4,
    // ],
    walking: &[
        UNIT4,
        UNIT5,
    ],
    attack: &[
        UNIT4_ATTACK1,
        UNIT4_ATTACK1,
        UNIT4_ATTACK2,
    ],

    speed: 2,
    health: 32,
    range: 0,
    damage: 3,
};

pub const GUNMAN: &UnitType = &UnitType {
    width: UNIT4_WIDTH,
    height: UNIT4_HEIGHT,

    // standing: &[
    //     UNIT4,
    // ],
    walking: &[
        UNIT4,
        UNIT5,
    ],
    attack: &[
        UNIT4_GUN1,
        UNIT4_GUN2,
    ],

    speed: 4,
    health: 8,
    range: 32,
    damage: 5,
};
