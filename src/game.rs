mod unit_types;

use std::collections::VecDeque;

use crate::{
    sprites::{BASE_WIDTH, UNIT4_WIDTH},
    *,
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Action {
    Stand,
    Walk,
    Attack,
}

pub struct Unit {
    animation_frame: u8,
    health: u8,
    x: i32,
    action: Action,
}

pub struct Game {
    // frame: u32,
    cam_x: i32,
    button_1: bool,
    player_units: VecDeque<Unit>,
    enemy_units: VecDeque<Unit>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            // frame: 0,
            cam_x: 256 - SCREEN_SIZE as i32,
            button_1: false,
            player_units: VecDeque::new(),
            enemy_units: VecDeque::from([
                Unit {
                    animation_frame: 0,
                    health: 64,
                    x: 256 - BASE_WIDTH as i32 - 5 - UNIT4_WIDTH as i32,
                    action: Action::Stand,
                },
                Unit {
                    animation_frame: 0,
                    health: 64,
                    x: 256 - BASE_WIDTH as i32 - 5 - UNIT4_WIDTH as i32,
                    action: Action::Stand,
                },
                Unit {
                    animation_frame: 0,
                    health: 64,
                    x: 256 - BASE_WIDTH as i32 - 5 - UNIT4_WIDTH as i32,
                    action: Action::Stand,
                },
            ]),
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };

        let prev_button_1 = self.button_1;

        self.button_1 = gamepad & BUTTON_1 != 0;

        let button_1_up = prev_button_1 && !self.button_1;

        // camera movement
        {
            const RANGE: i32 = 16;

            let mouse_x = (unsafe { *MOUSE_X } as i32).min(SCREEN_SIZE as i32).max(0);

            if mouse_x < RANGE {
                let change = -(mouse_x - RANGE) / 2;

                self.cam_x = 0.max(self.cam_x - change);
            } else if mouse_x > SCREEN_SIZE as i32 - RANGE {
                let change = (mouse_x - (SCREEN_SIZE as i32 - RANGE)) / 2;

                self.cam_x = (256 - SCREEN_SIZE as i32).min(self.cam_x + change);
            }
        }

        // spawn unit
        if button_1_up {
            self.player_units.push_back(Unit {
                animation_frame: 0,
                health: 12,
                x: sprites::BASE_WIDTH as i32 + 4,
                action: Action::Stand,
            });
        }

        set_draw_colors(0x4320);

        // base
        blit(
            sprites::BASE,
            -self.cam_x,
            (SCREEN_SIZE - sprites::BASE_HEIGHT) as i32,
            sprites::BASE_WIDTH,
            sprites::BASE_HEIGHT,
            sprites::BASE_FLAGS,
        );

        // enemy base
        blit(
            sprites::BASE,
            256 - sprites::BASE_WIDTH as i32 - self.cam_x,
            (SCREEN_SIZE - sprites::BASE_HEIGHT) as i32,
            sprites::BASE_WIDTH,
            sprites::BASE_HEIGHT,
            sprites::BASE_FLAGS | BLIT_FLIP_X,
        );

        // player units
        {
            let mut last_unit: Option<&mut Unit> = None;

            for unit in self.player_units.iter_mut() {
                update_unit(unit, last_unit, &mut self.enemy_units, true, self.cam_x);

                last_unit = Some(unit);
            }
        }

        // enemy units
        {
            let mut last_unit: Option<&mut Unit> = None;

            for unit in self.enemy_units.iter_mut() {
                update_unit(unit, last_unit, &mut self.player_units, false, self.cam_x);

                last_unit = Some(unit);
            }
        }

        // self.frame += 1;
    }
}

fn update_unit(
    unit: &mut Unit,
    last_unit: Option<&mut Unit>,
    enemies: &mut VecDeque<Unit>,
    forward: bool,
    cam_x: i32,
) {
    use Action::*;

    if enemies
        .front()
        .map(|e| {
            if forward {
                e.x - unit.x < 16
            } else {
                unit.x - e.x < 16
            }
        })
        .unwrap_or(false)
    {
        // enemy in range

        if unit.action != Attack {
            unit.action = Attack;
            unit.animation_frame = 0;
        }

        let enemy = enemies.front_mut().unwrap();

        if unit.animation_frame % 8 == 7 {
            enemy.health = enemy.health.saturating_sub(4);

            if enemy.health == 0 {
                enemies.pop_front();
            }

            // todo: kill if zero
        }

        unit.animation_frame = (unit.animation_frame + 1) % 16;
    } else if last_unit
        .map(|u| {
            if forward {
                u.x > unit.x + sprites::UNIT4_WIDTH as i32 + 5
            } else {
                u.x + sprites::UNIT4_WIDTH as i32 + 5 < unit.x
            }
        })
        .unwrap_or(true)
    {
        // previous unit out of the way

        if unit.action != Walk {
            unit.action = Walk;
            unit.animation_frame = 0;
        }

        if unit.animation_frame % 8 == 7 {
            if forward {
                unit.x += 1;
            } else {
                unit.x -= 1;
            }
        }

        unit.animation_frame = (unit.animation_frame + 1) % 16;
    } else {
        unit.action = Stand;
        unit.animation_frame = 0;
    }

    let animation_frame = unit.animation_frame / 8;

    let x = unit.x - cam_x;
    const Y: i32 = (SCREEN_SIZE - sprites::UNIT4_HEIGHT) as i32;

    let sprite = match (unit.action, animation_frame) {
        (Walk, 0) => sprites::UNIT4,
        (Walk, 1) => sprites::UNIT5,
        (Attack, 0) => sprites::UNIT4_ATTACK1,
        (Attack, 1) => sprites::UNIT4_ATTACK2,
        _ => sprites::UNIT4, // Standing
    };

    let flags = if forward {
        sprites::UNIT4_FLAGS
    } else {
        sprites::UNIT4_FLAGS | BLIT_FLIP_X
    };

    blit(
        sprite,
        x,
        Y,
        sprites::UNIT4_WIDTH,
        sprites::UNIT4_HEIGHT,
        flags,
    );
}
