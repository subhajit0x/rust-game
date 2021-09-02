use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition, Direction, RectangleBorder};
use crate::assets::Assets;
use ggez::graphics::Color;

pub struct Tower {
    render_position: GridPosition,
    borders: RectangleBorder,
    level: i16,
}

impl Tower {
    pub fn new(position: GridPosition) -> Self {
        let pos: (f32, f32) = position.into();
        let offset_render_position = GridPosition::new(pos.0 - 2.0, pos.1 - 4.0);

        Tower {
            render_position: offset_render_position,
            borders: RectangleBorder::new((pos.0 - 1.0, pos.1 - 1.0).into(), (pos.0 + 1.0, pos.1 + 1.0).into()),
            level: 0,
        }
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult {
        let current_position: (f32, f32) = self.render_position.into();
        let tower_sprite = assets.get_tower_image();
        let tower_sprite_dest: ggez::mint::Point2<f32> = self.render_position.into();
        let tower_draw_params = graphics::DrawParam::new().dest(tower_sprite_dest);

        let upgrade_position: GridPosition = (current_position.0, current_position.1 + 5 as f32).into();
        let upgrade_str = format!("Price: {}", self.level * 100);
        let upgrade_display = graphics::Text::new((upgrade_str));
        let upgrade_dest: ggez::mint::Point2<f32> = upgrade_position.into();

        graphics::draw(ctx, tower_sprite, tower_draw_params);
        graphics::draw(ctx, &upgrade_display, (upgrade_dest, 0.0, Color::BLACK))?;
        Ok(())
    }
}
