use sdl2::render::Texture;
use sdl2::render::TextureQuery;

pub struct TextTexture<'a> {
    pub texture: Texture<'a>,
    pub query: TextureQuery
}

impl<'a> TextTexture<'a> {
    pub fn new<'b>(texture: Texture<'a>, query: TextureQuery) -> Self {
        TextTexture {
            texture: texture,
            query: query
        }
    }
}

unsafe impl<'a> Send for TextTexture<'a> {}
unsafe impl<'a> Sync for TextTexture<'a> {}