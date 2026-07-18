use crate::bullet::{Bullet, BulletType};
use crate::entity::Entity;
use crate::perf::{FrameTimer, TickTimer};
use crate::player::{PLAYER_MAX_POS, PLAYER_MIN_POS, Player};
use crate::uid::{EntityUid, UidGenerator};

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub const WINDOW_PIXEL_WIDTH: usize = 150;
pub const WINDOW_PIXEL_HEIGHT: usize = 100;

const PIXEL_SIZE: usize = 6;
const TICK_DURATION: Duration = Duration::from_millis(10);

pub struct Game {
    window: Window,
    uid_generator: UidGenerator,
    world: World,
}

struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Rusty Invaders",
            WINDOW_PIXEL_WIDTH * PIXEL_SIZE,
            WINDOW_PIXEL_HEIGHT * PIXEL_SIZE,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();
        window.set_target_fps(0);

        let mut uid_generator = UidGenerator::default();

        let player = Player::new(uid_generator.generate());

        let mut world = World::new();
        world.entities.push(Entity::Player(player));

        let game = Self {
            window,
            uid_generator,
            world,
        };

        game
    }

    pub fn run(&mut self) {
        let mut frame_timer = FrameTimer::new();
        let mut tick_timer = TickTimer::new(TICK_DURATION);

        loop {
            while tick_timer.consume_tick() {
                self.handle_input();
                self.update();
            }

            self.render();

            if let Some(fps) = frame_timer.end_frame() {
                println!("FPS: {fps:.0}");
            }
        }
    }

    fn handle_input(&mut self) {
        let keys = self.window.get_keys();
        let mut can_shoot = false;
        let mut bullet_id = None;
        let mut left_pressed = false;
        let mut right_pressed = false;

        {
            let player = self.player();

            for key in keys {
                match key {
                    Key::Left | Key::A => left_pressed = true,
                    Key::Right | Key::D => right_pressed = true,
                    Key::Space => can_shoot = player.can_shoot(),
                    _ => {}
                }
            }

            match (left_pressed, right_pressed) {
                (true, false) => player.set_velocity((-1, 0)),
                (false, true) => player.set_velocity((1, 0)),
                _ => player.set_velocity((0, 0)),
            }
        }

        if can_shoot {
            bullet_id = Some(self.uid_generator.generate());
        }

        if let Some(bullet_id) = bullet_id {
            let player = self.player();
            let bullet = Bullet::new(
                bullet_id,
                (player.pos().0 + player.dims().0 / 2, player.pos().1),
                BulletType::Player,
            );
            self.world.entities.push(Entity::Bullet(bullet));
        }
    }

    fn update(&mut self) {
        let mut dead_entity_uids: Vec<EntityUid> = Vec::new();

        for entity in self.world.entities.iter_mut() {
            let velocity = entity.velocity();

            let mut new_pos = (
                entity.pos().0 as isize + velocity.0,
                entity.pos().1 as isize + velocity.1,
            );

            if entity.is_player() {
                if new_pos.0 < PLAYER_MIN_POS as isize {
                    new_pos.0 = PLAYER_MIN_POS as isize;
                } else if new_pos.0 > PLAYER_MAX_POS as isize {
                    new_pos.0 = PLAYER_MAX_POS as isize;
                }
            } else {
                if new_pos.0 < 0 || new_pos.0 >= WINDOW_PIXEL_WIDTH as isize {
                    dead_entity_uids.push(entity.uid());
                }
            }

            entity.set_pos((new_pos.0 as usize, new_pos.1 as usize));
        }

        self.world
            .entities
            .retain(|entity| !dead_entity_uids.contains(&entity.uid()));
    }

    fn render(&mut self) {
        let size = self.window.get_size();
        let mut buffer = raqote::DrawTarget::new(size.0 as i32, size.1 as i32);

        for entity in &self.world.entities {
            entity.render(&mut buffer);
        }

        self.window
            .update_with_buffer(buffer.get_data(), size.0, size.1)
            .unwrap();
    }

    fn player(&mut self) -> &mut Player {
        self.world
            .entities
            .get_mut(0)
            .expect("Player not found")
            .as_player_mut()
            .expect("Player not found")
    }
}
