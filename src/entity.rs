use crate::bullet::Bullet;
use crate::player::Player;
use crate::render::Sprite;

pub enum Entity {
    Player(Player),
    // Alien(Alien),
    Bullet(Bullet),
}

impl Entity {
    pub fn pos(&self) -> (usize, usize) {
        match self {
            Entity::Player(player) => player.pos(),
            Entity::Bullet(bullet) => bullet.pos(),
        }
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        match self {
            Entity::Player(player) => player.set_pos(pos),
            Entity::Bullet(bullet) => bullet.set_pos(pos),
        }
    }

    pub fn velocity(&self) -> (i32, i32) {
        match self {
            Entity::Player(player) => player.velocity(),
            Entity::Bullet(bullet) => bullet.velocity(),
        }
    }

    pub fn set_velocity(&mut self, velocity: (i32, i32)) {
        match self {
            Entity::Player(player) => player.set_velocity(velocity),
            Entity::Bullet(bullet) => bullet.set_velocity(velocity),
        }
    }

    pub fn sprite(&self) -> &Sprite {
        match self {
            Entity::Player(player) => player.sprite(),
            Entity::Bullet(bullet) => bullet.sprite(),
        }
    }
}
