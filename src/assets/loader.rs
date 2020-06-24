use crate::*;
use crate::rendering::*;

use std::fs::File;

pub struct AssetLoader<'a> {
    quad_ctx: &'a mut miniquad::Context,
    rendering_engine: &'a mut RenderingEngine,
}
impl<'a> AssetLoader<'a> {
    pub fn new(quad_ctx: &'a mut miniquad::Context, rendering_engine: &'a mut RenderingEngine) -> Self {
        AssetLoader {
            rendering_engine,
            quad_ctx,
        }
    }

    pub fn file<T: Into<String>>(&self, file_path: T) -> Result<File, EmeraldError> {
        let file_path: String = file_path.into();
        let file = File::open(file_path)?;

        Ok(file)
    }

    pub fn aseprite<T: Into<String>>(&mut self, path_to_texture: T, path_to_animations: T) -> Result<Aseprite, EmeraldError> {
        self.rendering_engine.aseprite(&mut self.quad_ctx, path_to_texture.into(), path_to_animations.into())
    }

    pub fn sprite(&mut self, path: &str) -> Result<Sprite, EmeraldError> {
        self.rendering_engine.sprite(&mut self.quad_ctx, path)
    }

    pub fn label<T: Into<String>>(&mut self, text: T, font_key: FontKey) -> Result<Label, EmeraldError> {
        let mut label = Label::default();

        label.font = font_key;
        label.text = text.into();

        Ok(Label::default())
    }

    pub fn font(&mut self, path: &str, font_size: u16) -> Result<FontKey, EmeraldError> {
        self.rendering_engine.font(&mut self.quad_ctx, path, font_size)
    }
}