use managers::{FontManager, TextureManager};
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use common::{TextTexture, FontDetails};
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;

pub struct ResourceFacade<'l> {
    font_manager: FontManager<'l>,
    texture_manager: TextureManager<'l, WindowContext>,
    texture_creator: &'l TextureCreator<WindowContext>
}

impl<'l> ResourceFacade<'l> {
    pub fn new(font_context: &'l Sdl2TtfContext, texture_creator: &'l TextureCreator<WindowContext>) -> Self {
        let font_manager = FontManager::new(&font_context);
        let texture_manager = TextureManager::new(&texture_creator);

        ResourceFacade {
            font_manager,
            texture_manager,
            texture_creator
        }
    }

    pub fn build_text<'a>(&'a mut self, text: &str, font_details: &FontDetails, color: &Color) -> TextTexture<'a> {
        let font = self.font_manager.load(font_details).unwrap();
        let text_render = font.render(text);
        let text_surface = text_render.solid(*color).unwrap();
        let text_texture = self.texture_creator.create_texture_from_surface(text_surface).unwrap();
        let text_query = text_texture.query();

        return TextTexture::new(text_texture, text_query);
    }
}