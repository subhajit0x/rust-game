mod nexus;
mod config;
mod tower;
mod enemy;
mod movement_helpers;
// mod assets;

use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};
use crate::config::{MILLIS_PER_UPDATE, SCREEN_SIZE};
use crate::nexus::Nexus;
use crate::enemy::Enemy;
use std::collections::VecDeque;

struct GameState {
    nexus: Nexus,
    enemies: VecDeque<Enemy>,
    score: f32,
    ticks: i32,
    gameover: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            nexus: Nexus::new((16, 19).into(), 3),
            enemies: VecDeque::new(),
            score: 0.0,
            ticks: 0,
            gameover: false,
            last_update: Instant::now(),
        })
    }

    pub fn spawn_enemy(mut self) {
        let enemy = Enemy::new(10);
        self.enemies.push_back(enemy);
    }

    pub fn kill_enemy(mut self) {
        self.enemies.pop_front();
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.gameover {
                // Check if any new enemies have hit the nexus - if so reduce its health
                // if the health is leq than 0 stop the game
                if self.ticks % 5 == 0 {
                    let enemy = Enemy::new(10);
                    self.enemies.push_back(enemy);
                    // self.spawn_enemy();
                }

                for enemy in self.enemies.iter_mut() {
                    enemy.update();
                }
            }
            self.last_update = Instant::now();
            self.ticks = self.ticks + 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        self.nexus.draw(ctx)?;
        for enemy in self.enemies.iter() {
            enemy.draw(ctx)?;
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "Hristiyan Genchev")
        .window_setup(ggez::conf::WindowSetup::default().title("Game!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .build()
        .expect("Failed to build ggez context");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
