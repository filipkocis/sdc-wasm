use web_sys::CanvasRenderingContext2d;

pub trait Drawable {
    fn draw(&self, context: &CanvasRenderingContext2d);
}

impl<T: Drawable> Drawable for Vec<T> {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.iter().for_each(|c| c.draw(context));
    }
}

impl<T: Drawable> Drawable for &[T] {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.iter().for_each(|c| c.draw(context));
    }
}
