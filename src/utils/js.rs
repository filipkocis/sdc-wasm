use wasm_bindgen::prelude::*;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn get_element_by_id(id: &str) -> web_sys::Element {
    document()
        .get_element_by_id(id)
        .expect(&format!("no element with id '{}' found", id))
}

pub fn canvas(element_id: &str) -> web_sys::HtmlCanvasElement {
    get_element_by_id(element_id)
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn context_2d(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
