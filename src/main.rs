mod nexus;
mod config;
mod tower;
mod enemy;
mod movement_helpers;
mod assets;
mod score_board;

use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};
use crate::config::{MILLIS_PER_UPDATE, SCREEN_SIZE, GRID_SIZE, GRID_CELL_SIZE};
use crate::nexus::Nexus;
use crate::enemy::Enemy;
use crate::assets::Assets;
use std::collections::VecDeque;
use std::{env, fs};
use std::path;
use ggez::graphics::Color;
use serde_json;
use crate::movement_helpers::{GridPosition, RectangleBorder};
use ggez::input::mouse::MouseButton;
use crate::tower::Tower;
use std::cmp::min;
use crate::score_board::ScoreBoard;

struct GameState {
    assets: Assets,
    map_json: serde_json::Value,
    nexus: Nexus,
    enemies: VecDeque<Enemy>,
    towers: Vec<Tower>,
    score: f32,
    ticks: i32,
    gameover: bool,
    last_update: Instant,
    score_board: ScoreBoard,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::new(ctx)?;
        let map_json_file = fs::File::open(get_resource_path("map_layout.json"))
            .expect("file should open read only");
        let map_json: serde_json::Value = serde_json::from_reader(map_json_file)
            .expect("file should be proper JSON");

        let score_board = ScoreBoard::new();
        let nexus: Nexus = Nexus::new(10);
        let towers: Vec<Tower> = vec![
            Tower::new((16, 8).into()),
            Tower::new((32, 8).into()),
            Tower::new((48, 8).into()),
            Tower::new((52, 16).into()),
            Tower::new((32, 15).into()),
            Tower::new((32, 24).into()),
            Tower::new((48, 24).into()),
        ];

        Ok(GameState {
            assets,
            map_json,
            nexus,
            enemies: VecDeque::new(),
            score_board,
            towers,
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

fn draw_map(assets: &mut Assets, ctx: &mut Context, map_json: &mut serde_json::Value) {
    for x in 0..GRID_SIZE.0 {
        for y in 0..GRID_SIZE.1 {
            let key = format!("{x}_{y}", x = x, y = y);
            let image = assets.get_tile_image(map_json[key]["sprite"].to_string());
            let dest: ggez::mint::Point2<f32> = GridPosition::new(x as f32, y as f32).into();
            // let offset: ggez::mint::Point2<f32> = GridPosition::new(4, 4).into();
            let drawparams = graphics::DrawParam::new()
                .dest(dest);
            // .offset(offset);
            graphics::draw(ctx, image, drawparams);
        }
    }
}

fn get_resources_dir() -> path::PathBuf {
    let resources_dir = if let Ok(resources_dir) = env::var("RESOURCES_DIR") {
        println!("manifest_dir: {}", resources_dir);
        let mut path = path::PathBuf::from(resources_dir);
        path
    } else {
        path::PathBuf::from("./resources")
    };

    resources_dir
}

fn get_resource_path(resource_name: &str) -> path::PathBuf {
    let resources_dir = get_resources_dir();
    let mut resources_dir_cloned = resources_dir.clone();
    resources_dir_cloned.push(resource_name);
    resources_dir_cloned
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.gameover {
                // update enemies
                for enemy in self.enemies.iter_mut() {
                    enemy.update();
                }

                let mut total_damage: i32 = 0;
                for tower in self.towers.iter() {
                    total_damage += tower.get_damage();
                }

                // for now all enemies will be with one speed
                // when this logic is changed make sure to update this code
                // DAMAGE ALL FRONT ENEMIES
                while total_damage > 0 && !self.enemies.is_empty() {
                    let front_enemy = self.enemies.front_mut().unwrap();
                    let front_enemy_health: i32 = front_enemy.get_health();
                    let health_to_reduce: i32 = min(front_enemy_health, total_damage);
                    front_enemy.reduce_health(health_to_reduce);

                    total_damage -= health_to_reduce;

                    if !front_enemy.is_alive() {
                        self.enemies.pop_front();
                    }
                }

                // spawn the next enemy if its time to do so
                if self.ticks % 7 == 0 {
                    let enemy = Enemy::new(10);
                    self.enemies.push_back(enemy);
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
            let map_json = &mut self.map_json;
            draw_map(assets, ctx, map_json);

            for enemy in self.enemies.iter() {
                enemy.draw(ctx, assets)?;
            }

            for tower in self.towers.iter() {
                tower.draw(ctx, assets)?;
            }

            self.nexus.draw(ctx, assets)?;
            self.score_board.draw(ctx, assets, 3, 3, 1000)?;
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        println!("_button: {:?}, _x: {}, _y: {}", _button, _x, _y);
        let click_pos: GridPosition = (_x / GRID_CELL_SIZE.0 as f32, _y / GRID_CELL_SIZE.0 as f32).into();
        for tower in self.towers.iter_mut() {
            if tower.is_clicking_on(click_pos) {
                tower.upgrade();
            }
        }
    }
}

fn main() -> GameResult {
    let resources_dir = get_resources_dir();

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "Hristiyan Genchev")
        .window_setup(ggez::conf::WindowSetup::default().title("Game!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .add_resource_path(resources_dir)
        .build()
        .expect("Failed to build ggez context");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
