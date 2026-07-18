use crate::entity::Entity;
use crate::perf::{FrameTimer, TickTimer};
use crate::player::{PLAYER_MAX_POS, PLAYER_MIN_POS, Player};

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub const WINDOW_PIXEL_WIDTH: usize = 150;
pub const WINDOW_PIXEL_HEIGHT: usize = 100;

const PIXEL_SIZE: usize = 6;
const TICK_DURATION: Duration = Duration::from_millis(10);

pub struct Game {
    window: Window,
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

        let player = Player::new();

        let mut world = World::new();
        world.entities.push(Entity::Player(player));

        let game = Self { window, world };

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
        let player = self.player();

        for key in keys {
            match key {
                Key::Left => player.set_velocity((1, 0)),
                Key::Right => player.set_velocity((-1, 0)),
                Key::Space => {
                    println!("Shoot!!");
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        for entity in &mut self.world.entities {
            let velocity = entity.velocity();

            let mut new_pos = (
                entity
                    .pos()
                    .0
                    .checked_sub_signed(velocity.0 as isize)
                    .unwrap_or(0),
                entity
                    .pos()
                    .1
                    .checked_sub_signed(velocity.1 as isize)
                    .unwrap_or(0),
            );

            if entity.is_player() {
                if new_pos.0 < PLAYER_MIN_POS {
                    new_pos.0 = PLAYER_MIN_POS;
                } else if new_pos.0 > PLAYER_MAX_POS {
                    new_pos.0 = PLAYER_MAX_POS;
                }
            }

            entity.set_pos(new_pos);
            entity.set_velocity((0, 0));
        }
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
