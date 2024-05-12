use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::prelude::*;
use web_sys::window;

use crate::{car::Car, Drawable};

pub struct Player {
    pub car: Rc<RefCell<Car>>,
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            car: Rc::new(RefCell::new(Car::default())),
        };
        player.add_controls_listeners();

        player
    }

    pub fn update(&self) {
        self.car.borrow_mut().update();
    }

    pub fn add_controls_listeners(&self) {
        {
            let car = self.car.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                let mut car = car.borrow_mut();

                match event.key().as_str() {
                    "ArrowUp" | "w" => car.controls.forward = true,
                    "ArrowDown" | "s" => car.controls.backward = true,
                    "ArrowLeft" | "a" => car.controls.left = true,
                    "ArrowRight" | "d" => car.controls.right = true,
                    " " => car.controls.brake = true,
                    _ => {}
                }
            });
            if let Some(window) = window() {
                let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        }

        {
            let car = self.car.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                let mut car = car.borrow_mut();

                match event.key().as_str() {
                    "ArrowUp" | "w" => car.controls.forward = false,
                    "ArrowDown" | "s" => car.controls.backward = false,
                    "ArrowLeft" | "a" => car.controls.left = false,
                    "ArrowRight" | "d" => car.controls.right = false,
                    " " => car.controls.brake = false,
                    _ => {}
                }
            });
            if let Some(window) = window() {
                let _ = window.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        }

    }
}

impl Drawable for Player {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.car.borrow().draw(context);
    }
}
