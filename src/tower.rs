use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition, Direction, RectangleBorder};
use crate::assets::Assets;

pub struct Tower {
    render_position: GridPosition,
    borders: RectangleBorder,
    level: i16,
}

impl Tower {
    pub fn new(position: GridPosition) -> Self {
        let pos: (i16, i16) = position.into();
        let offset_render_position = GridPosition::new(pos.0 - 2, pos.1 - 4);

        Tower {
            render_position: offset_render_position,
            borders: RectangleBorder::new((pos.0 - 1, pos.1 - 1).into(), (pos.0 + 1, pos.1 + 1).into()),
            level: 0,
        }
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult {
        let image = assets.get_tower_image();
        let dest: ggez::mint::Point2<f32> = self.render_position.into();
        // let offset: ggez::mint::Point2<f32> = GridPosition::new(4, 4).into();
        let drawparams = graphics::DrawParam::new()
            .dest(dest);
        // .offset(offset);
        graphics::draw(ctx, image, drawparams);
        Ok(())
    }
}
