mod visual;
mod ai;
mod world;
mod utils;
mod game;

pub use world::*;
pub use ai::*;
pub use visual::*;
pub use utils::*;

use game::Game;

// use std::cell::RefCell;
// use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    panic_utils::set_panic_hook();

    let game = Game::new();

    game.start();

    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();
    //
    // let game_canvas = Rc::new(RefCell::new(Canvas::from("gameCanvas")));
    // let node_canvas = Rc::new(RefCell::new(Canvas::from("nodeCanvas")));
    //
    // let points = Rc::new(RefCell::new(Vec::new()));
    //
    // {
    //     let context = game_canvas.borrow().element.clone();
    //     let points = points.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         points.borrow_mut().push(geo::Point::from_event(&event, &context));
    //         console_log!("{} {}", event.client_x(), event.client_y());
    //     });
    //     game_canvas.borrow_mut().element.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }
    //
    // let cars = Rc::new(RefCell::new(Vec::new()));
    // for _ in 0..10000 {
    //     cars.borrow_mut().push(Car::default())
    // }
    //
    // let mut i = 0;
    // *g.borrow_mut() = Some(Closure::new(move || {
    //     let game_canvas = game_canvas.borrow();
    //     let node_canvas = node_canvas.borrow();
    //
    //     game_canvas.resize();
    //     node_canvas.resize();
    //
    //     let mut cars = cars.borrow_mut();
    //     game_canvas.draw(&cars);
    //     cars.iter_mut().for_each(|c| c.update());
    //
    //     let points = points.borrow();
    //     for i in 0..points.len() {
    //         let a = &points[i];
    //         let b = &points[(i + 1) % points.len()]; 
    //
    //         let a = geo::Point::new(a.x, a.y);
    //         let b = geo::Point::new(b.x, b.y);
    //
    //         let line = geo::Line::new(a, b);
    //         line.draw(&game_canvas.context);
    //     }
    //
    //
    //     if false {
    //         js::get_element_by_id("rustOutput").set_text_content(Some("All done!"));
    //
    //         let _ = f.borrow_mut().take();
    //         return;
    //     }
    //
    //     i += 1;
    //     let text = format!("requestAnimationFrame has been called {} times.", i);
    //     js::get_element_by_id("rustOutput").set_text_content(Some(&text));
    //
    //     js::request_animation_frame(f.borrow().as_ref().unwrap());
    // }));
    //
    // js::request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(()) 
}
