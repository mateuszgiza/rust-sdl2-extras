use adapters::{CanvasAdapter, GetCanvasError};
use common::GameTime;
use std::time::Duration;
use sdl2::{
    video::{Window, WindowContext},
    render::{Canvas, TextureCreator}
};
use specs::World;

pub trait WorldExt {
    fn proceed_on_canvas<F>(&self, canvas_action: F) -> Result<(), GetCanvasError> where F: Fn(&mut Canvas<Window>);
    fn update_delta_time(&mut self, new_delta: Duration);
    fn get_texture_creator(&mut self) -> Result<TextureCreator<WindowContext>, GetCanvasError>;
}

impl WorldExt for World {
    fn proceed_on_canvas<F>(&self, canvas_action: F) -> Result<(), GetCanvasError>
    where F: Fn(&mut Canvas<Window>) {
        self.write_resource::<CanvasAdapter>().proceed(canvas_action)
    }

    fn update_delta_time(&mut self, new_delta: Duration) {
        self.write_resource::<GameTime>().set_delta(new_delta);
    }

    fn get_texture_creator(&mut self) -> Result<TextureCreator<WindowContext>, GetCanvasError> {
        self.write_resource::<CanvasAdapter>().texture_creator()
    }
}