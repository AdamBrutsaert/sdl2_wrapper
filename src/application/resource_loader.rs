use std::cell::RefCell;

use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};

pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;

    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;

    fn load(&'l self, path: &str) -> Result<Texture, String> {
        self.load_texture(path)
    }
}

impl<'l, T> ResourceLoader<'l, RefCell<Texture<'l>>> for TextureCreator<T> {
    type Args = str;

    fn load(&'l self, _details: &str) -> Result<RefCell<Texture>, String> {
        self.create_texture_target(PixelFormatEnum::RGBA8888, 100, 100)
            .map(RefCell::new)
            .map_err(|e| e.to_string())
    }
}
