use sdl2::render::{ Texture, TextureCreator };
use sdl2::ttf::{ Font, Sdl2TtfContext };

pub mod resource_manager;
use self::resource_manager::{ ResourceManager, FontDetails };

pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;