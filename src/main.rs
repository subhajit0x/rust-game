mod nexus;
mod config;
mod tower;
mod enemy;
mod movement_helpers;
mod assets;

use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};
use crate::config::{MILLIS_PER_UPDATE, SCREEN_SIZE};
use crate::nexus::Nexus;
use crate::enemy::Enemy;
use crate::assets::Assets;
use std::collections::VecDeque;
use std::env;
use std::path;
use ggez::graphics::Color;
use crate::movement_helpers::GridPosition;

struct GameState {
    assets: Assets,
    nexus: Nexus,
    enemies: VecDeque<Enemy>,
    score: f32,
    ticks: i32,
    gameover: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::new(ctx)?;
        let (width, height) = graphics::drawable_size(ctx);
        println!("width: {}, height: {}", width, height);

        Ok(GameState {
            assets,
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

fn draw_map(assets: &mut Assets, ctx: &mut Context) -> GameResult {
    let image = assets.get_tile_image("".to_string());
    let dest: ggez::mint::Point2<f32> = GridPosition::new(63, 31).into();
    // let offset: ggez::mint::Point2<f32> = GridPosition::new(4, 4).into();

    let drawparams = graphics::DrawParam::new()
        .dest(dest);
        // .offset(offset);
    graphics::draw(ctx, image, drawparams)
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.gameover {
                if self.ticks % 5 == 0 {
                    let enemy = Enemy::new(10);
                    self.enemies.push_back(enemy);
                }

                for enemy in self.enemies.iter_mut() {
                    enemy.update();
                }

                // Check if any new enemies have hit the nexus - if so reduce its health
                // if the health is leq than 0 stop the game
                // if self.enemies.
            }
            self.last_update = Instant::now();
            self.ticks = self.ticks + 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        {
            let assets = &mut self.assets;
            draw_map(assets, ctx)?;
            for enemy in self.enemies.iter() {
                enemy.draw(ctx)?;
            }
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(resources_dir) = env::var("RESOURCES_DIR") {
        println!("manifest_dir: {}", resources_dir);
        let mut path = path::PathBuf::from(resources_dir);
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "Hristiyan Genchev")
        .window_setup(ggez::conf::WindowSetup::default().title("Game!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .add_resource_path(resource_dir)
        .build()
        .expect("Failed to build ggez context");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
