use crate::*;
use crate::world::*;
use crate::rendering::*;
use crate::rendering::texture::{Texture};
use crate::rendering::font::FontKey;

use std::fs::File;
use std::io::Read as StdIoRead;

use miniquad::{
    BlendFactor, BlendState, BlendValue, Equation,
    Pipeline, PipelineParams,
    Bindings, BufferType, BufferLayout,
    Context, Buffer, VertexFormat,
    VertexAttribute, Shader};
use glam::{Vec2, Vec4, Mat4};
use legion::prelude::{Schedulable, Query, SystemBuilder, Read, Write, IntoQuery};
use std::collections::HashMap;
use fontdue::{Font, FontSettings};

pub struct RenderingEngine {
    settings: RenderSettings,
    pipeline: Pipeline,
    textures: HashMap<TextureKey, Texture>,
    fonts: HashMap<FontKey, Font>,
    font_atlases: HashMap<FontKey, Texture>,
}
impl RenderingEngine {
    pub fn new(mut ctx: &mut Context, settings: RenderSettings) -> Self {
        let shader = Shader::new(ctx, VERTEX, FRAGMENT, META).unwrap();

        let mut params = PipelineParams::default();
        params.depth_write = true;
        params.color_blend = Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
        );
        params.alpha_blend = Some(BlendState::new(
            Equation::Add,
            BlendFactor::Zero,
            BlendFactor::One)
        );

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            params,
        );

        let mut textures: HashMap<TextureKey, Texture> = HashMap::new();
        let fonts = HashMap::new();
        let font_atlases = HashMap::new();

        let default_texture = Texture::default(&mut ctx).unwrap();
        textures.insert(TextureKey::default(), default_texture);

        RenderingEngine {
            settings,
            pipeline,
            textures,
            fonts,
            font_atlases,
        }
    }

    #[inline]
    pub fn update(&mut self, mut ctx: &mut Context, world: &mut World) {
        let sprite_query = <(Read<Sprite>, Read<Position>)>::query();

        ctx.begin_default_pass(Default::default());
        ctx.clear(Some(self.settings.background_color.percentage()), None, None);
        ctx.apply_pipeline(&self.pipeline);

        for (sprite, position) in sprite_query.iter(&mut world.inner) {
            self.render_sprite(&mut ctx, &sprite, &position);
        }
        
        ctx.end_render_pass();
        ctx.commit_frame();
    }

    // fn render_color_rect(&mut self, ctx: &mut Context, color_rect: &ColorRect) {}

    #[inline]
    fn render_sprite(&mut self, ctx: &mut Context, sprite: &Sprite, position: &Position) {
        let texture = self.textures.get(&sprite.texture_key).unwrap();
        let mut target = Rectangle::new(
            sprite.target.x / texture.width as f32,
            sprite.target.y / texture.height as f32,
            (sprite.target.x + sprite.target.width) / texture.width as f32,
            (sprite.target.y + sprite.target.height) / texture.height as f32,
        );

        if sprite.target.is_zero_sized() {
            target = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        }

        let model = Mat4::identity();
        let uniforms = Uniforms {
            model,
            offset: Vec2::new(position.x, position.y),
            view_size: Vec2::new(800.0, 600.0),
            z_index: sprite.z_index as f32,
            target: Vec4::new(target.x, target.y, target.width, target.height)
        };

        ctx.apply_bindings(&texture.bindings);
        ctx.apply_uniforms(&uniforms);
        ctx.draw(0, 6, 1);
    }

    // fn render_label(&mut self, ctx: &mut Context, label: &Label, position: &Position) {
    //     // Get font texture here
    //     // Render texture font at target characters in sequence
    // }

    #[inline]
    pub fn aseprite<T: Into<String>>(&mut self,
            mut ctx: &mut Context,
            texture_file: File,
            texture_file_path: T,
            animation_file: File,
            _animation_file_path: T
        ) -> Result<Aseprite, EmeraldError> {
        let sprite = self.sprite(&mut ctx, texture_file, texture_file_path)?;

        Aseprite::new(sprite, animation_file)
    }

    #[inline]
    pub fn sprite<T: Into<String>>(&mut self, mut ctx: &mut Context, file: File, path: T) -> Result<Sprite, EmeraldError> {
        let key = self.texture(&mut ctx, file, path)?;

        Ok(Sprite::from_texture(key))
    }

    #[inline]
    pub fn texture<T: Into<String>>(&mut self, mut ctx: &mut Context, file: File, path: T) -> Result<TextureKey, EmeraldError> {
        let path: String = path.into();
        let key = TextureKey::new(path.clone());

        if !self.textures.contains_key(&key) {
            let texture = Texture::new(&mut ctx, file)?;
            self.textures.insert(key.clone(), texture);
        }

        Ok(key)
    }

    pub fn pack_texture(&mut self, mut ctx: &mut Context, name: &str, bytes: Vec<u8>) {
        let texture = Texture::from_png_bytes(&mut ctx, bytes.as_slice()).unwrap();
        let key = TextureKey::new(name.to_string());
        
        self.textures.insert(key, texture);
    }

    #[inline]
    pub fn font(&mut self, mut ctx: &mut Context, mut file: File, path: &str, font_size: u16) -> Result<FontKey, EmeraldError> {
        let key = FontKey::new(path, font_size);

        if self.fonts.contains_key(&key) {
            return Ok(key);
        }

        let mut font_data = Vec::new();
        file.read_to_end(&mut font_data)?;

        let font = Font::from_bytes(font_data.as_slice(), FontSettings::default())?;
        self.fonts.insert(key.clone(), font);

        // Create texture here big enough for fuckin regular letters shit or something idk man
        // Characters are hard
        // Just do the 0..26 for now
        // Just load texture to the engine textures, then point at it
        let size: u16 = 128;
        let mut bytes = Vec::with_capacity((size * size) as usize);

        for _ in 0..(size * size) {
            bytes.push(0xFF);
        }

        let font_texture = Texture::from_rgba8(&mut ctx, size, size, &bytes)?;
        let texture_key = TextureKey::new(path);
        self.textures.insert(texture_key, font_texture);

        Ok(key)
    }
}