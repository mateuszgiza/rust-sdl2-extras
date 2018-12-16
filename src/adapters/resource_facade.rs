use managers::{FontManager, TextureManager};
use sdl2::video::WindowContext;
use sdl2::render::{TextureCreator, Texture};
use common::{TextTexture, FontDetails};
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use std::rc::Rc;

pub struct ResourceFacade<'l> {
    font_manager: Option<FontManager<'l>>,
    texture_manager: Option<TextureManager<'l, WindowContext>>,
    texture_creator: Option<&'l TextureCreator<WindowContext>>
}

unsafe impl<'l> Send for ResourceFacade<'l> {}
unsafe impl<'l> Sync for ResourceFacade<'l> {}

impl<'l> Default for ResourceFacade<'l> {
    fn default() -> Self { ResourceFacade{
        font_manager: None,
        texture_manager: None,
        texture_creator: None
    } }
}

impl<'l> ResourceFacade<'l> {
    pub fn new(font_context: &'l Sdl2TtfContext, texture_creator: &'l TextureCreator<WindowContext>) -> Self {
        let font_manager = FontManager::new(&font_context);
        let texture_manager = TextureManager::new(&texture_creator);

        ResourceFacade {
            font_manager: Some(font_manager),
            texture_manager: Some(texture_manager),
            texture_creator: Some(texture_creator)
        }
    }

    // pub fn load_font(&mut self, font_name: )

    pub fn build_text<'a>(&'a mut self, text: &str, font_details: &FontDetails, color: &Color) -> TextTexture<'a> {
        let font = self.font_manager.as_mut().unwrap().load(font_details).unwrap();
        let text_render = font.render(text);
        let text_surface = text_render.solid(*color).unwrap();
        let text_texture = self.texture_creator.unwrap().create_texture_from_surface(text_surface).unwrap();
        let text_query = text_texture.query();

        return TextTexture::new(text_texture, text_query);
    }

    pub fn get_texture(&mut self, texture_name: &String) -> Result<Rc<Texture>, String> {
        self.texture_manager.as_mut().unwrap().load(&texture_name)
    }
}