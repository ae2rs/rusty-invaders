use std::time::Instant;

use crate::asset::sprite::ship::{
    HEIGHT as SHIP_HEIGHT, SCHEMA as SHIP_SCHEMA, WIDTH as SHIP_WIDTH,
};
use crate::bullet::{Bullet, BulletType};
use crate::game::{WINDOW_PIXEL_HEIGHT, WINDOW_PIXEL_WIDTH};
use crate::render::Sprite;

pub const PLAYER_MIN_POS: usize = 2;
pub const PLAYER_MAX_POS: usize = WINDOW_PIXEL_WIDTH - SHIP_WIDTH - 2;
const PLAYER_SHOT_INTERVAL: u128 = 200;

#[derive(Debug)]
pub struct Player {
    pos: (usize, usize),
    dims: (usize, usize),
    sprite: Sprite,
    velocity: (isize, isize),

    last_shot: Instant,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: (PLAYER_MIN_POS, WINDOW_PIXEL_HEIGHT - SHIP_HEIGHT - 2),
            dims: (SHIP_WIDTH, SHIP_HEIGHT),
            sprite: Sprite::from_asset(
                &SHIP_SCHEMA[..]
                    .iter()
                    .map(|inner| inner as &[u8])
                    .collect::<Vec<&[u8]>>(),
            ),
            velocity: (0, 0),

            last_shot: Instant::now(),
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    pub fn velocity(&self) -> (isize, isize) {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: (isize, isize)) {
        self.velocity = velocity;
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn shoot(&mut self) -> Option<Bullet> {
        if self.last_shot.elapsed().as_millis() > PLAYER_SHOT_INTERVAL {
            self.last_shot = Instant::now();
            return Some(Bullet::new(
                (self.pos.0 + self.dims.0 / 2, self.pos.1),
                BulletType::Player,
            ));
        }
        None
    }
}
