use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct CanvasAdapter {
    canvas: Option<Canvas<Window>>
}

unsafe impl Send for CanvasAdapter {}
unsafe impl Sync for CanvasAdapter {}

impl Default for CanvasAdapter {
    fn default() -> Self { CanvasAdapter::new(None) }
}

impl<'a> CanvasAdapter {
    pub fn new(canvas: Option<Canvas<Window>>) -> Self {
        CanvasAdapter {
            canvas: canvas
        }
    }

    pub fn borrow(&mut self) -> Option<&mut Canvas<Window>> {
        return self.canvas.as_mut();
    }

    pub fn proceed<F>(&mut self, canvas_action: F) where F: Fn(&mut Canvas<Window>) {
        let mut canvas = self.borrow().unwrap();
        canvas_action(&mut canvas);
    }
}