use web_sys::CanvasRenderingContext2d;

pub trait Drawable {
    fn draw(&self, context: &CanvasRenderingContext2d);
}
