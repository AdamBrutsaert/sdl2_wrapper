use sdl2::image::LoadTexture;
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
