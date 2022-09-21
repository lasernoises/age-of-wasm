mod unit_types;

use std::collections::VecDeque;

use crate::{
    sprites::{BASE_WIDTH, UNIT4_WIDTH},
    *,
};

use self::unit_types::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Action {
    // Standing could depend on what the previous unit is doing.
    // Stand,
    Walk,
    Attack,
}

pub struct Unit {
    unit_type: &'static UnitType,
    animation_frame: u8,
    walking_frame: u8,
    health: u8,
    x: i32,
    action: Action,
}

impl Unit {
    fn new(unit_type: &'static UnitType, player: bool) -> Self {
        Unit {
            unit_type,
            animation_frame: 0,
            walking_frame: 0,
            health: unit_type.health,
            x: if player {
                sprites::BASE_WIDTH as i32 + 4
            } else {
                256 - BASE_WIDTH as i32 - 5 - UNIT4_WIDTH as i32
            },
            action: Action::Walk,
        }
    }
}

pub struct Game {
    // frame: u32,
    cam_x: i32,
    button_1: bool,
    button_2: bool,
    player_units: VecDeque<Unit>,
    enemy_units: VecDeque<Unit>,
    player_base_health: u8,
    enemy_base_health: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            // frame: 0,
            cam_x: 256 - SCREEN_SIZE as i32,
            button_1: false,
            button_2: false,
            player_units: VecDeque::new(),
            enemy_units: VecDeque::from([
                Unit::new(GUNMAN, false),
                Unit::new(BOXER, false),
                Unit::new(GUNMAN, false),
                Unit::new(BOXER, false),
            ]),
            player_base_health: 255,
            enemy_base_health: 255,
        }
    }

    pub fn update(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };

        let prev_button_1 = self.button_1;
        let prev_button_2 = self.button_2;

        self.button_1 = gamepad & BUTTON_1 != 0;
        self.button_2 = gamepad & BUTTON_2 != 0;

        let button_1_up = prev_button_1 && !self.button_1;
        let button_2_up = prev_button_2 && !self.button_2;

        if self.enemy_base_health == 0 {
            set_draw_colors(0x2);
            text("You Won!!!", 12, 12);
            return;
        } else if self.player_base_health == 0 {
            set_draw_colors(0x2);
            text("You Lost!!!", 12, 12);
            return;
        }

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
            self.player_units.push_back(Unit::new(BOXER, true));
        }

        if button_2_up {
            self.player_units.push_back(Unit::new(GUNMAN, true));
        }

        // player units
        {
            let mut last_unit: Option<&mut Unit> = None;

            for unit in self.player_units.iter_mut() {
                update_unit(unit, last_unit, &mut self.enemy_units, &mut self.enemy_base_health, true, self.cam_x);

                last_unit = Some(unit);
            }
        }

        // enemy units
        {
            let mut last_unit: Option<&mut Unit> = None;

            for unit in self.enemy_units.iter_mut() {
                update_unit(unit, last_unit, &mut self.player_units, &mut self.player_base_health, false, self.cam_x);

                last_unit = Some(unit);
            }
        }

        set_draw_colors(0x4320);

        // player base
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

        // health bars

        set_draw_colors(0x2);

        let player_health_bar_height = 64 * self.player_base_health as u32 / 255;
        let enemy_health_bar_height = 64 * self.enemy_base_health as u32 / 255;

        rect(
            -self.cam_x + 12,
            32,
            8,
            64 - player_health_bar_height,
        );

        rect(
            256 - 12 - self.cam_x,
            32,
            8,
            64 - enemy_health_bar_height,
        );

        set_draw_colors(0x4);

        rect(
            -self.cam_x + 12,
            32 + 64 - player_health_bar_height as i32,
            8,
            player_health_bar_height,
        );

        rect(
            256 - 12 - self.cam_x,
            32 + 64 - enemy_health_bar_height as i32,
            8,
            enemy_health_bar_height,
        );
    }
}

fn update_unit(
    unit: &mut Unit,
    last_unit: Option<&mut Unit>,
    enemies: &mut VecDeque<Unit>,
    enemy_base_health: &mut u8,
    forward: bool,
    cam_x: i32,
) {
    use Action::*;

    if enemies
        .front()
        .map(|e| if forward {
            unit.x + unit.unit_type.width as i32 + unit.unit_type.range as i32 >= e.x
        } else {
            unit.x - unit.unit_type.range as i32 <= e.x + e.unit_type.width as i32
        })
        .unwrap_or(false)
    {
        // enemy in range

        if unit.action != Attack {
            unit.action = Attack;
            unit.animation_frame = 0;
        }

        let enemy = enemies.front_mut().unwrap();

        let frames = (8 * unit.unit_type.attack.len()) as u8;

        if unit.animation_frame == frames - 1 {
            enemy.health = enemy.health.saturating_sub(unit.unit_type.damage);

            if enemy.health == 0 {
                enemies.pop_front();
            }

            // todo: death animation
        }

        unit.animation_frame = (unit.animation_frame + 1) % frames;
    } else if if forward {
        unit.x + unit.unit_type.width as i32 >= 255 - BASE_WIDTH as i32 - unit.unit_type.range as i32
    } else {
        unit.x <= BASE_WIDTH as i32 + unit.unit_type.range as i32
    } {
        // enemy base in range

        if unit.action != Attack {
            unit.action = Attack;
            unit.animation_frame = 0;
        }

        let frames = (8 * unit.unit_type.attack.len()) as u8;

        if unit.animation_frame == frames - 1 {
            *enemy_base_health = enemy_base_health.saturating_sub(unit.unit_type.damage);
        }

        unit.animation_frame = (unit.animation_frame + 1) % frames;
    } else if last_unit
        .map(|u| {
            if forward {
                u.x > unit.x + unit.unit_type.width as i32 + 5
            } else {
                u.x + u.unit_type.width as i32 + 5 < unit.x
            }
        })
        .unwrap_or(true)
    {
        // previous unit out of the way

        if unit.action != Walk {
            unit.action = Walk;
            unit.animation_frame = 0;
            unit.walking_frame = 0;
        }

        unit.walking_frame += 1;

        if unit.walking_frame == unit.unit_type.speed {
            if forward {
                unit.x += 1;
            } else {
                unit.x -= 1;
            }

            unit.walking_frame = 0;
        }

        unit.animation_frame = (unit.animation_frame + 1) % 16;
    } else {
        if unit.action != Walk {
            unit.action = Walk;
            unit.animation_frame = 0;
        }

        // TODO: Standing animation
    }

    let animation_frame = unit.animation_frame / 8;

    let x = unit.x - cam_x;
    let y = (SCREEN_SIZE - unit.unit_type.height) as i32;

    let sprite = match unit.action {
        Walk => unit.unit_type.walking[animation_frame as usize],
        Attack => unit.unit_type.attack[animation_frame as usize],
        _ => unit.unit_type.walking[animation_frame as usize], // Standing
    };

    let flags = if forward {
        BLIT_2BPP
    } else {
        BLIT_2BPP | BLIT_FLIP_X
    };

    set_draw_colors(0x4320);

    blit(
        sprite,
        x,
        y,
        sprites::UNIT4_WIDTH,
        sprites::UNIT4_HEIGHT,
        flags,
    );

    if unit.health < unit.unit_type.health {
        let bar_size = 12 * unit.health as u32 / unit.unit_type.health as u32;

        let x = unit.x + unit.unit_type.width as i32 / 2 - 6;

        set_draw_colors(0x4);
        rect(x - cam_x, SCREEN_SIZE as i32 - unit.unit_type.height as i32 - 4, bar_size, 2);

        set_draw_colors(0x2);
        rect(x + bar_size as i32 - cam_x, SCREEN_SIZE as i32 - unit.unit_type.height as i32 - 4, 12 - bar_size, 2);
    }
}
