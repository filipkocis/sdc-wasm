use wasm_bindgen::JsValue;

use crate::{geo::{Line, Point}, helpers, Drawable};

pub struct Sensor {
    pub ray: Line,
    pub reading: f64,
    pub length: f64,
}

pub struct Sensors {
    pub sensors: Vec<Sensor>, 
    pub x: f64,    
    pub y: f64,
    pub angle: f64,
    pub spread: f64,
}

impl Sensors {
    pub fn new(x: f64, y: f64, count: u32, length: f64, spread: f64, angle: f64) -> Sensors {
        let mut sensors = Vec::new();
        
        for i in 0..count {
            let angle = angle + helpers::lerpf(-spread / 2.0, spread / 2.0, i as f64 / (count - 1) as f64);
            let ray = Line::new(Point::new(x, y), Point::new(x + angle.cos() * length, y + angle.sin() * length)); 

            sensors.push(Sensor { ray, reading: 0.0, length });
        }

        Sensors {
            sensors,
            x, y,
            angle,
            spread,
        }
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
        self.sensors.iter_mut().for_each(|s| {
            s.ray.translate(x, y)
        })
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
        self.sensors.iter_mut().for_each(|s| {
            s.ray.rotate(angle)
        })
    }
}

impl Drawable for Sensors {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str("yellow"));
        context.set_line_width(1.0);
    
        for sensor in self.sensors.iter() {
            context.begin_path();
            context.move_to(sensor.ray.start.x, sensor.ray.start.y);
            context.line_to(sensor.ray.end.x, sensor.ray.end.y);

            context.stroke();
        }

        context.set_stroke_style(&JsValue::from_str("black"));

        for sensor in self.sensors.iter() {
            context.begin_path();
            context.arc(sensor.ray.end.x, sensor.ray.end.y, 2.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.fill();
            
            context.stroke();
        }
    }
}
