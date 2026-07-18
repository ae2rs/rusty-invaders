use crate::alien::Enemy;
use crate::entity::Entity;
use crate::perf::{FrameTimer, TickTimer};
use crate::player::Player;

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

    fn handle_input(&mut self) {}

    fn update(&mut self) {}

    fn render(&mut self) {
        let size = self.window.get_size();
        let mut buffer = raqote::DrawTarget::new(size.0 as i32, size.1 as i32);

        self.window
            .update_with_buffer(buffer.get_data(), size.0, size.1)
            .unwrap();
    }
}
