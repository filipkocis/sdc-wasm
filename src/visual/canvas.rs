use web_sys::*;

use crate::*;

pub struct Canvas {
    pub id: String,
    pub element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn from(element_id: &str) -> Canvas {
        let element = js::canvas(element_id);
        let context = js::context_2d(&element);

        Canvas {
            id: element_id.to_string(),
            element,
            context,
        }
    }

    pub fn resize(&self) {
        let computed_style = self.element.get_bounding_client_rect();

        self.element.set_width(computed_style.width() as u32);
        self.element.set_height(computed_style.height() as u32);
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.element.width().into(), self.element.height().into())
    }

    pub fn draw(&self, drawables: &[impl Drawable]) {
        drawables.iter().for_each(|d| d.draw(&self.context)) 
    }
}
