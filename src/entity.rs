use crate::bullet::Bullet;
use crate::player::Player;

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

    pub fn velocity(&self) -> (isize, isize) {
        match self {
            Entity::Player(player) => player.velocity(),
            Entity::Bullet(bullet) => bullet.velocity(),
        }
    }

    pub fn set_velocity(&mut self, velocity: (isize, isize)) {
        match self {
            Entity::Player(player) => player.set_velocity(velocity),
            Entity::Bullet(bullet) => bullet.set_velocity(velocity),
        }
    }

    pub fn render(&self, buffer: &mut raqote::DrawTarget) {
        match self {
            Entity::Player(player) => player.sprite().render(player.pos(), buffer),
            Entity::Bullet(bullet) => bullet.sprite().render(bullet.pos(), buffer),
        }
    }
}

// Player specific
impl Entity {
    pub fn is_player(&self) -> bool {
        match self {
            Entity::Player(_) => true,
            _ => false,
        }
    }

    pub fn as_player_mut(&mut self) -> Option<&mut Player> {
        match self {
            Entity::Player(player) => Some(player),
            _ => None,
        }
    }
}
