use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;

pub(crate) struct Assets {
    tiles: HashMap<String, graphics::Image>,
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
        let mut tiles: HashMap<String, graphics::Image> = HashMap::new();
        tiles.insert("6".to_string(), graphics::Image::new(ctx, "/06.png")?);
        tiles.insert("14".to_string(), graphics::Image::new(ctx, "/14.png")?);
        tiles.insert("19".to_string(), graphics::Image::new(ctx, "/19.png")?);
        tiles.insert("33".to_string(), graphics::Image::new(ctx, "/33.png")?);
        tiles.insert("44".to_string(), graphics::Image::new(ctx, "/44.png")?);
        tiles.insert("48".to_string(), graphics::Image::new(ctx, "/48.png")?);
        tiles.insert("57".to_string(), graphics::Image::new(ctx, "/57.png")?);
        tiles.insert("62".to_string(), graphics::Image::new(ctx, "/62.png")?);
        tiles.insert("70".to_string(), graphics::Image::new(ctx, "/70.png")?);
        tiles.insert("71".to_string(), graphics::Image::new(ctx, "/71.png")?);

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
            tiles,
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

    pub(crate) fn get_tile_image(&mut self, mut sprite_name: String) -> &graphics::Image {
        println!("sprite_name: {}", sprite_name);
        return if let Some(x) = self.tiles.get_mut(&sprite_name) {
            x
        } else {
            &mut self.nexus_image
            // panic!("The sprite {} was not found in the assets list", sprite_name);
        };
    }
}
