use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext}
};

pub type GetCanvasError = String;

pub struct CanvasAdapter {
    canvas: Option<Canvas<Window>>
}

unsafe impl Send for CanvasAdapter {}
unsafe impl Sync for CanvasAdapter {}

impl Default for CanvasAdapter {
    fn default() -> Self { CanvasAdapter::new(None) }
}

impl CanvasAdapter {
    pub fn new(canvas: Option<Canvas<Window>>) -> Self {
        CanvasAdapter {
            canvas: canvas
        }
    }

    pub fn borrow(&mut self) -> Option<&mut Canvas<Window>> {
        return self.canvas.as_mut();
    }

    pub fn proceed<F>(&mut self, canvas_action: F) -> Result<(), GetCanvasError>
    where F: Fn(&mut Canvas<Window>) {
        let mut canvas = self.get_canvas()?;
        canvas_action(&mut canvas);
        Ok(())
    }

    pub fn texture_creator(&mut self) -> Result<TextureCreator<WindowContext>, GetCanvasError> {
        let canvas = self.get_canvas()?;
        Ok(canvas.texture_creator())
    }

    fn get_canvas(&mut self) -> Result<&mut Canvas<Window>, GetCanvasError> {
        match self.borrow() {
            Some(canvas) => Ok(canvas),
            None => Err("Canvas not loaded".into())
        }
    }
}