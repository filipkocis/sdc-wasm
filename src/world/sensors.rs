use wasm_bindgen::JsValue;

use crate::{geo::{Line, Point, Polygon}, helpers, Drawable, Road};

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

    pub fn update(&mut self, road: &Road) {
        self.reset();

        self.check_polygon(&road.hitbox);
    }

    pub fn reset(&mut self) {
        self.sensors.iter_mut().for_each(|s| s.reading = 0.0);
    }

    pub fn check(&mut self, obstacles: &Vec<Line>) {
        if obstacles.is_empty() { return }
        let last = obstacles.last().unwrap();

        for sensor in self.sensors.iter_mut() {
            for obstacle in obstacles.iter() {
                // skip the last line which is the finish line
                if obstacle.matches_both_points(last) { continue }

                let intersection = sensor.ray.get_intersection(obstacle);

                if let Some(intersection) = intersection {
                    if !intersection.intersects { continue }

                    if sensor.reading == 0.0 || intersection.offset < sensor.reading {
                        sensor.reading = intersection.offset;
                    }
                }
            }
        }
    }

    pub fn check_polygon(&mut self, obstacle: &Polygon) {
        self.check(&obstacle.lines());
        // for sensor in self.sensors.iter_mut() {
        //     for line in obstacle.lines() {
        //         let intersection = sensor.ray.get_intersection(&line);
        //
        //         if let Some(intersection) = intersection {
        //             sensor.reading = intersection.offset;
        //
        //             if sensor.reading < 0.0 || sensor.reading > 1.0 {
        //                 sensor.reading = 0.0;
        //             }
        //
        //             break;
        //         }
        //     }
        // }
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
        // context.set_stroke_style(&JsValue::from_str("yellow"));
        // context.set_line_width(1.0);
    
        // for sensor in self.sensors.iter() {
        //     context.begin_path();
        //     context.move_to(sensor.ray.start.x, sensor.ray.start.y);
        //     context.line_to(sensor.ray.end.x, sensor.ray.end.y);
        //
        //     context.stroke();
        // }
        
        context.set_line_width(1.0);

        for sensor in self.sensors.iter() {
            context.set_stroke_style(&JsValue::from_str("lime"));

            let reading_point = Point::new(
                sensor.ray.start.x + (sensor.reading * sensor.length) * sensor.ray.angle().cos(), 
                sensor.ray.start.y + (sensor.reading * sensor.length) * sensor.ray.angle().sin()
            ); 

            context.begin_path();
            context.move_to(sensor.ray.start.x, sensor.ray.start.y);
            context.line_to(reading_point.x, reading_point.y); 
            context.stroke();

            context.set_stroke_style(&JsValue::from_str("black"));
            context.set_fill_style(&JsValue::from_str("lime"));
            context.begin_path();
            context.arc(reading_point.x, reading_point.y, 2.5, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.fill();
            context.stroke();
        }
    }
}
