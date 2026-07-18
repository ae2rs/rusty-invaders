use crate::asset::sprite::shot::SCHEMA as SHOT_SCHEMA;
use crate::render::Sprite;
use crate::{game::WINDOW_PIXEL_HEIGHT, traits::Kinetic};

pub enum BulletType {
    Player,
    Enemy,
}

pub struct Bullet {
    pos: (usize, usize),
    dims: (usize, usize),
    velocity: (i32, i32),
    sprite: Sprite,

    bullet_type: BulletType,
}

impl Bullet {
    pub fn new(pos: (usize, usize), bullet_type: BulletType) -> Self {
        Self {
            pos,
            dims: (1, 2),
            velocity: (0, 1),
            sprite: Sprite::from_asset(
                &SHOT_SCHEMA[..]
                    .iter()
                    .map(|inner| inner as &[u8])
                    .collect::<Vec<&[u8]>>(),
            ),
            bullet_type,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    pub fn velocity(&self) -> (i32, i32) {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: (i32, i32)) {
        self.velocity = velocity;
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}

impl Kinetic for Bullet {
    fn translate(&mut self) -> bool {
        match self.bullet_type {
            BulletType::Player => {
                if self.pos.1 == 0 {
                    false
                } else {
                    self.pos.1 = self.pos.1 - 1;
                    true
                }
            }
            BulletType::Enemy => {
                if self.pos.1 == WINDOW_PIXEL_HEIGHT - 1 {
                    false
                } else {
                    self.pos.1 = self.pos.1 + 1;
                    true
                }
            }
        }
    }
}
