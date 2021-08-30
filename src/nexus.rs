use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition};

pub struct Nexus {
    position: GridPosition,
    health: i16,
}

impl Nexus {
    pub fn new(position: GridPosition, health: i16) -> Self {
        Nexus {
            position,
            health,
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.position.into(),
                graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}
