use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition};
use crate::assets::Assets;

pub struct Nexus {
    position: GridPosition,
    render_position: GridPosition,
    health: i16,
}

impl Nexus {
    pub fn new(health: i16) -> Self {
        Nexus {
            position: (16, 19).into(),
            render_position: (13.5, 16.5).into(),
            health,
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult {
        let nexus_sprite = assets.get_nexus_image();
        let nexus_sprite_dest: ggez::mint::Point2<f32> = self.render_position.into();
        let nexus_draw_params = graphics::DrawParam::new().dest(nexus_sprite_dest);

        graphics::draw(ctx, nexus_sprite, nexus_draw_params);
        Ok(())
    }
}
