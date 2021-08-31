use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

pub(crate) struct Assets {
    nexus_image: graphics::Image,
    heart_image: graphics::Image,
    enemy_image: graphics::Image,
    tower_image: graphics::Image,
    // font: graphics::Font,
    rocket_image: graphics::Image,
    // rocket_sound: audio::Source,
    // nexus_hit_sound: audio::Source,
    // enemy_hit_sound: audio::Source,
}

impl Assets {
    pub(crate) fn new(ctx: &mut Context) -> GameResult<Assets> {
        let nexus_image = graphics::Image::new(ctx, "/71.png")?;
        let heart_image = graphics::Image::new(ctx, "/71.png")?;
        let enemy_image = graphics::Image::new(ctx, "/71.png")?;
        let tower_image = graphics::Image::new(ctx, "/71.png")?;
        let rocket_image = graphics::Image::new(ctx, "/71.png")?;

        // let font = graphics::Font::new(ctx, "/font.ttf")?;
        // let rocket_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let nexus_hit_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let enemy_hit_sound = audio::Source::new(ctx, "/pew.ogg")?;

        Ok(Assets {
            nexus_image,
            heart_image,
            enemy_image,
            tower_image,
            // font,
            rocket_image,
            // rocket_sound,
            // nexus_hit_sound,
            // enemy_hit_sound,
        })
    }

    pub(crate) fn get_tile_image(&mut self, tile_sprite: String) -> &mut graphics::Image{
        return &mut self.nexus_image;
    }
}
