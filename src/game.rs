use crate::enemy::Enemy;
use crate::perf::FrameTimer;
use crate::player::Player;
use crate::render::{Pixel, Sprite};
use crate::shot::Shot;
use crate::traits::Kinetic;

use minifb::{Key, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder};
use std::time::Duration;

pub const WINDOW_PIXEL_WIDTH: usize = 150;
pub const WINDOW_PIXEL_HEIGHT: usize = 100;

const PIXEL_SIZE: usize = 6;
const TICK_DURATION: u64 = 10;

pub struct Game {
    window: Window,
    screen: Vec<Vec<Option<Pixel>>>,
}

impl Game {
    pub fn new() -> Self {
        // minifb window creation
        let window = Window::new(
            "Rusty Invaders",
            WINDOW_PIXEL_WIDTH * PIXEL_SIZE,
            WINDOW_PIXEL_HEIGHT * PIXEL_SIZE,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();

        let mut screen = Vec::with_capacity(WINDOW_PIXEL_HEIGHT);
        for _ in 0..WINDOW_PIXEL_HEIGHT {
            screen.push(Vec::from([None; WINDOW_PIXEL_WIDTH]));
        }

        let game = Self { window, screen };

        game
    }

    pub fn run(&mut self) {
        let mut frame_timer = FrameTimer::new(Duration::from_millis(TICK_DURATION));

        let mut player = Player::new();
        let mut pshots = Vec::new();
        let mut enemies = Vec::with_capacity(10);

        for i in 0..11 {
            enemies.push(Enemy::new((2 + (Enemy::ENEMY_SPRITE_WIDTH + 2) * i, 2)));
            enemies.push(Enemy::new((
                5 + (Enemy::ENEMY_SPRITE_WIDTH + 2) * i,
                2 + Enemy::ENEMY_SPRITE_HEIGHT + 2,
            )));
            enemies.push(Enemy::new((
                2 + (Enemy::ENEMY_SPRITE_WIDTH + 2) * i,
                2 + (Enemy::ENEMY_SPRITE_HEIGHT + 2) * 2,
            )));
        }

        loop {
            frame_timer.begin_frame();

            self.handle_input(&mut player, &mut pshots);
            self.update(&mut pshots);
            self.render(&mut player, &mut pshots, &mut enemies);

            if let Some(fps) = frame_timer.end_frame() {
                println!("FPS: {fps:.1}");
            }
        }
    }

    fn handle_input(&mut self, player: &mut Player, pshots: &mut Vec<Shot>) {
        for key in self.window.get_keys() {
            match key {
                Key::Left => player.left(),
                Key::Right => player.right(),
                Key::Space => {
                    if let Some(shot) = player.shoot() {
                        pshots.push(shot);
                    }
                }
                _ => {}
            }
        }
    }

    fn update(&self, pshots: &mut Vec<Shot>) {
        pshots.retain_mut(|shot| shot.translate());
    }

    fn render(&mut self, player: &mut Player, pshots: &mut Vec<Shot>, enemy: &mut Vec<Enemy>) {
        self.draw_screen(player, pshots, enemy);
        self.render_frame();
    }

    fn render_frame(&mut self) {
        let size = self.window.get_size();
        let mut buffer = DrawTarget::new(size.0 as i32, size.1 as i32);

        for (row, pixels) in self.screen.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate().filter(|p| p.1.is_some()) {
                let pixel = pixel.unwrap();
                let mut pb = PathBuilder::new();
                pb.rect(
                    (col * PIXEL_SIZE) as f32,
                    (row * PIXEL_SIZE) as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                );
                let path = pb.finish();
                let pixel_color = pixel.color();
                buffer.fill(&path, &pixel_color.source(), &DrawOptions::new());
            }
        }

        self.window
            .update_with_buffer(buffer.get_data(), size.0, size.1)
            .unwrap();
    }

    fn draw_sprite_at(&mut self, pos: (usize, usize), sprite: &Sprite) {
        for (row, pixels) in sprite.pixels().iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                if pixel.is_some() {
                    self.screen[pos.1 + row][pos.0 + col] = Some(pixel.unwrap());
                }
            }
        }
    }

    fn draw_screen(&mut self, player: &Player, pshots: &mut Vec<Shot>, enemy: &mut Vec<Enemy>) {
        self.clear_screen();

        self.draw_sprite_at(player.pos(), player.sprite());

        for e in enemy {
            self.draw_sprite_at(e.pos(), &e.sprite());
        }

        for s in pshots {
            self.draw_sprite_at(s.pos(), &s.sprite());
        }
    }

    fn clear_screen(&mut self) {
        self.screen.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|p| p.is_some())
                .for_each(|p| *p = None)
        });
    }
}
