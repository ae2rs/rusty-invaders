use crate::asset::sprite::enemy::{
    HEIGHT as ENEMY_HEIGHT, SCHEMA as ENEMY_SCHEMA, WIDTH as ENEMY_WIDTH,
};
use crate::render::Sprite;

pub struct Enemy {
    pos: (usize, usize),
    sprite: Sprite,
}

impl Enemy {
    pub const ENEMY_SPRITE_HEIGHT: usize = ENEMY_HEIGHT;
    pub const ENEMY_SPRITE_WIDTH: usize = ENEMY_WIDTH;

    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            sprite: Sprite::from_asset(
                &ENEMY_SCHEMA[..]
                    .iter()
                    .map(|inner| inner as &[u8])
                    .collect::<Vec<&[u8]>>(),
            ),
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn sprite(&self) -> Sprite {
        self.sprite.clone()
    }
}
