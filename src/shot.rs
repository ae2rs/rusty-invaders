use crate::asset::sprite::shot::SCHEMA as SHOT_SCHEMA;
use crate::render::Sprite;
use crate::{game::WINDOW_PIXEL_HEIGHT, traits::Kinetic};

pub enum ShotType {
    Player,
    #[expect(dead_code)]
    Enemy,
}

pub struct Shot {
    pos: (usize, usize),
    sprite: Sprite,
    shot_type: ShotType,
}

impl Shot {
    pub fn new(pos: (usize, usize), shot_type: ShotType) -> Self {
        Self {
            pos,
            sprite: Sprite::from_asset(
                &SHOT_SCHEMA[..]
                    .iter()
                    .map(|inner| inner as &[u8])
                    .collect::<Vec<&[u8]>>(),
            ),
            shot_type,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn sprite(&self) -> Sprite {
        self.sprite.clone()
    }
}

impl Kinetic for Shot {
    fn translate(&mut self) -> bool {
        match self.shot_type {
            ShotType::Player => {
                if self.pos.1 == 0 {
                    false
                } else {
                    self.pos.1 = self.pos.1 - 1;
                    true
                }
            }
            ShotType::Enemy => {
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
