use std::time::Duration;
use sdl2::video::Window;
use sdl2::render::Canvas;
use specs::World;
use adapters::CanvasAdapter;
use common::GameTime;

pub trait WorldExt {
    fn proceed_on_canvas<F>(&self, canvas_action: F) where F: Fn(&mut Canvas<Window>);
    fn update_delta_time(&mut self, new_delta: Duration);
}

impl WorldExt for World {
    fn proceed_on_canvas<F>(&self, canvas_action: F) where F: Fn(&mut Canvas<Window>) {
        self.write_resource::<CanvasAdapter>().proceed(canvas_action);
    }

    fn update_delta_time(&mut self, new_delta: Duration) {
        self.write_resource::<GameTime>().set_delta(new_delta);
    }
}