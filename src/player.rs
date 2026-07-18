use std::time::Instant;

use crate::asset::sprite::ship::{
    HEIGHT as SHIP_HEIGHT, SCHEMA as SHIP_SCHEMA, WIDTH as SHIP_WIDTH,
};
use crate::game::{WINDOW_PIXEL_HEIGHT, WINDOW_PIXEL_WIDTH};
use crate::render::Sprite;
use crate::shot::{Shot, ShotType};

const PLAYER_MIN_POS: usize = 2;
const PLAYER_MAX_POS: usize = WINDOW_PIXEL_WIDTH - SHIP_WIDTH - 2;

const PLAYER_SHOT_INTERVAL: u128 = 200;

#[derive(Debug)]
pub struct Player {
    pos: (usize, usize),
    dims: (usize, usize),
    sprite: Sprite,
    last_shot_time: Instant,
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
            last_shot_time: Instant::now(),
        }
    }

    pub fn left(&mut self) {
        if self.pos.0 > PLAYER_MIN_POS {
            self.pos.0 = self.pos.0 - 1;
        }
    }

    pub fn right(&mut self) {
        if self.pos.0 < PLAYER_MAX_POS {
            self.pos.0 = self.pos.0 + 1;
        }
    }

    pub fn shoot(&mut self) -> Option<Shot> {
        if self.last_shot_time.elapsed().as_millis() > PLAYER_SHOT_INTERVAL {
            self.last_shot_time = Instant::now();
            return Some(Shot::new(
                (self.pos.0 + self.dims.0 / 2, self.pos.1),
                ShotType::Player,
            ));
        }
        None
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn sprite(&self) -> Sprite {
        self.sprite.clone()
    }
}
