use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition, Direction};
use crate::assets::Assets;
use ggez::graphics::Color;

pub struct ScoreBoard {
    label_render_position: GridPosition,
    score_text_render_position: GridPosition,
    honey_text_render_position: GridPosition,
    honey_image_render_position: GridPosition,
    lives_text_render_position: GridPosition,
    lives_hearts_render_position: GridPosition,
}

impl ScoreBoard {
    pub fn new() -> Self {
        ScoreBoard {
            label_render_position: (21.0, 0.3).into(),
            score_text_render_position: (21.5, 1.5).into(),
            honey_text_render_position: (28.5, 1.5).into(),
            honey_image_render_position: (31.7, 1.1).into(),
            lives_text_render_position: (36.5, 1.5).into(),
            lives_hearts_render_position: (40., 1.4).into(),
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, score: i32, lives: i32, honey: i32) -> GameResult {
        {
            let label_sprite = assets.get_score_label_image();
            let label_sprite_dest: ggez::mint::Point2<f32> = self.label_render_position.into();
            let label_sprite_scale: ggez::mint::Point2<f32> = GridPosition::new(0.2, 0.1).into();
            let label_draw_params = graphics::DrawParam::new()
                .dest(label_sprite_dest)
                .scale(label_sprite_scale);

            graphics::draw(ctx, label_sprite, label_draw_params);
        }

        {
            let honey_sprite = assets.get_honey_image();
            let honey_sprite_dest: ggez::mint::Point2<f32> = self.honey_image_render_position.into();
            let honey_draw_params = graphics::DrawParam::new()
                .dest(honey_sprite_dest);

            graphics::draw(ctx, honey_sprite, honey_draw_params);
        }

        for i in 0..lives {
            let heart_sprite = assets.get_heart_image();
            let heart_sprite_dest: ggez::mint::Point2<f32> = GridPosition::new_from_move(self.lives_hearts_render_position, Direction::Right, i as f32 * 1.).into();
            let heart_draw_params = graphics::DrawParam::new()
                .dest(heart_sprite_dest);

            graphics::draw(ctx, heart_sprite, heart_draw_params);
        }

        let score_str = format!("Score: {}", score);
        let score_display = graphics::Text::new((score_str));
        let score_dest: ggez::mint::Point2<f32> = self.score_text_render_position.into();

        let honey_str = format!("Honey:   {}", honey);
        let honey_display = graphics::Text::new((honey_str));
        let honey_dest: ggez::mint::Point2<f32> = self.honey_text_render_position.into();

        let lives_str = format!("Lives: ");
        let lives_display = graphics::Text::new((lives_str));
            // .set_font(ggez::graphics::Font::default(), (1.1).into());
        let lives_dest: ggez::mint::Point2<f32> = self.lives_text_render_position.into();

        graphics::draw(ctx, &score_display, (score_dest, 0.0, Color::BLACK))?;
        graphics::draw(ctx, &honey_display, (honey_dest, 0.0, Color::BLACK))?;
        graphics::draw(ctx, &lives_display, (lives_dest, 0.0, Color::BLACK))?;
        Ok(())
    }
}
