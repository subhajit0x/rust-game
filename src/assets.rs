use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;

pub struct Assets {
    towers: HashMap<String, graphics::Image>,
    default_tower: graphics::Image,
    decors: HashMap<String, graphics::Image>,
    default_decor: graphics::Image,
    tiles: HashMap<String, graphics::Image>,
    default_tile: graphics::Image,
    enemies: HashMap<String, graphics::Image>,
    default_enemy: graphics::Image,
    score_label: graphics::Image,
    nexus_image: graphics::Image,
    // font: graphics::Font,
    // rocket_sound: audio::Source,
    // nexus_hit_sound: audio::Source,
    // enemy_hit_sound: audio::Source,
    honey_image: graphics::Image,
    heart_image: graphics::Image,
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

        let mut decors: HashMap<String, graphics::Image> = HashMap::new();
        decors.insert("101".to_string(), graphics::Image::new(ctx, "/101.png")?);
        decors.insert("102".to_string(), graphics::Image::new(ctx, "/102.png")?);
        decors.insert("103".to_string(), graphics::Image::new(ctx, "/103.png")?);
        decors.insert("104".to_string(), graphics::Image::new(ctx, "/104.png")?);
        decors.insert("105".to_string(), graphics::Image::new(ctx, "/105.png")?);

        let mut enemies: HashMap<String, graphics::Image> = HashMap::new();
        enemies.insert("slime_blue".to_string(), graphics::Image::new(ctx, "/slime_blue.png")?);
        enemies.insert("slime_green".to_string(), graphics::Image::new(ctx, "/slime_blue.png")?);
        enemies.insert("slime_orange".to_string(), graphics::Image::new(ctx, "/slime_blue.png")?);

        let mut towers: HashMap<String, graphics::Image> = HashMap::new();
        towers.insert("tower_1".to_string(), graphics::Image::new(ctx, "/tower_1.png")?);
        towers.insert("tower_3".to_string(), graphics::Image::new(ctx, "/tower_3.png")?);
        towers.insert("tower_5".to_string(), graphics::Image::new(ctx, "/tower_5.png")?);

        let default_tile = graphics::Image::new(ctx, "/default_tile.png")?;
        let default_decor = graphics::Image::new(ctx, "/101.png")?;
        let default_enemy = graphics::Image::new(ctx, "/default_enemy.png")?;
        let default_tower = graphics::Image::new(ctx, "/tower_disabled.png")?;
        let score_label = graphics::Image::new(ctx, "/score_label.png")?;
        let honey_image = graphics::Image::new(ctx, "/honey.png")?;
        let heart_image = graphics::Image::new(ctx, "/heart.png")?;
        let nexus_image = graphics::Image::new(ctx, "/nexus.png")?;

        // let font = graphics::Font::new(ctx, "/font.ttf")?;
        // let rocket_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let nexus_hit_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let enemy_hit_sound = audio::Source::new(ctx, "/pew.ogg")?;

        Ok(Assets {
            towers,
            default_tower,
            tiles,
            default_tile,
            decors,
            default_decor,
            enemies,
            default_enemy,
            score_label,
            nexus_image,
            honey_image,
            heart_image,
            // font,
            // rocket_sound,
            // nexus_hit_sound,
            // enemy_hit_sound,
        })
    }

    pub(crate) fn get_enemy_image(&mut self, mut sprite_name: String) -> &graphics::Image {
        return if let Some(x) = self.enemies.get_mut(&sprite_name) {
            x
        } else {
            &mut self.default_enemy
            // panic!("The sprite {} was not found in the assets list", sprite_name);
        };
    }

    pub(crate) fn get_nexus_image(&mut self) -> &graphics::Image {
        &self.nexus_image
    }

    pub(crate) fn get_score_label_image(&mut self) -> &graphics::Image {
        &self.score_label
    }

    pub(crate) fn get_honey_image(&mut self) -> &graphics::Image {
        &self.honey_image
    }

    pub(crate) fn get_heart_image(&mut self) -> &graphics::Image {
        &self.heart_image
    }

    pub(crate) fn get_tower_image(&mut self, level: i32) -> &graphics::Image {
        let sprite_name = if level >= 5 {
            "tower_5"
        } else if level >= 3 {
            "tower_3"
        } else if level >= 1 {
            "tower_1"
        } else {
            "tower_disabled"
        };

        return if let Some(x) = self.towers.get_mut(&sprite_name.to_string()) {
            x
        } else {
            &mut self.default_tower
            // panic!("The sprite {} was not found in the assets list", sprite_name);
        };
    }

    pub(crate) fn get_tile_image(&mut self, mut sprite_name: String) -> &graphics::Image {
        return if let Some(x) = self.tiles.get_mut(&sprite_name) {
            x
        } else {
            &mut self.default_tile
            // panic!("The sprite {} was not found in the assets list", sprite_name);
        };
    }

    pub(crate) fn get_decor_image(&mut self, mut decor_name: String) -> &graphics::Image {
        return if let Some(x) = self.decors.get_mut(&decor_name) {
            x
        } else {
            &mut self.default_decor
            // panic!("The sprite {} was not found in the assets list", sprite_name);
        };
    }
}
