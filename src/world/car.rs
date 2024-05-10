use wasm_bindgen::prelude::*;

use crate::Drawable;

pub struct Car {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub direction: f64,
    pub color: String,

    pub speed: f64,
    pub acceleration: f64,
    pub friction: f64,
}

impl Car {
    pub fn new(x: f64, y: f64, width: f64, height: f64, direction: f64, color: String, speed: f64, acceleration: f64, friction: f64) -> Car {
        Car {
            x, y, width, height, direction, color, speed, acceleration, friction
        }
    } 

    pub fn update(&mut self) {
        self.y += self.speed
    }
}

impl Drawable for Car {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str(self.color.as_str()));

        context.begin_path();
        context.move_to(self.x, self.y);
        context.arc(self.x, self.y, self.width, self.y, self.direction).unwrap();
        context.fill();

        // ctx.beginPath()
        //
        // ctx.moveTo(this.polygon[0].x, this.polygon[0].y)
        // for (let i = 1; i < this.polygon.length; i++) {
        //   ctx.lineTo(this.polygon[i].x, this.polygon[i].y)
        // }
        // ctx.fill()
    }
}

impl Default for Car{
    fn default() -> Car {
        Car {
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 100.0,
            direction: 0.0,
            color: String::from("blue"),
            speed: 10.0,
            acceleration: 0.0,
            friction: 0.0,
        } 
    }
}
