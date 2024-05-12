use web_sys::js_sys::Math;

use crate::{Drawable, geo::{Polygon, Point}};

pub struct Controls {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub brake: bool,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            forward: false,
            backward: false,
            left: false,
            right: false,
            brake: false,
        }
    }

    pub fn reset(&mut self) {
        self.forward = false;
        self.backward = false;
        self.left = false;
        self.right = false;
        self.brake = false;
    }

    pub fn update(&mut self, forward: bool, backward: bool, left: bool, right: bool, brake: bool) {
        self.forward = forward;
        self.backward = backward;
        self.left = left;
        self.right = right;
        self.brake = brake;
    }

    pub fn is_active(&self) -> bool {
        self.forward || self.backward || self.left || self.right || self.brake
    }

    pub fn is_moving(&self) -> bool {
        self.forward || self.backward
    }

    pub fn is_turning(&self) -> bool {
        self.left || self.right
    }
}

pub struct Car {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,

    pub speed: f64,
    pub max_speed: f64,
    pub acceleration: f64,
    pub friction: f64,

    pub polygons: Vec<Polygon>,
    pub controls: Controls,
    pub hitbox: Polygon,

    // pub sensors: Vec<Sensor>,    
    // pub brain: Brain,
    // pub fitness: f64,
    pub has_collided: bool,
}

impl Car {
    pub fn new(x: f64, y: f64, width: f64, height: f64, angle: f64, speed: f64, max_speed: f64, acceleration: f64, friction: f64) -> Car {
        let polygons = Car::generate_polygons(width, height, angle);
        let controls = Controls::new();

        let mut hitbox = Polygon::rectangle(x, y, height, width, angle);
        hitbox.fill_color = "transparent".to_owned();
        hitbox.scale(1.2, 1.2);

        Car {
            x, y, width, height, angle, 
            speed, max_speed, acceleration, friction, 
            polygons, controls, hitbox,
            has_collided: false,
        }
    } 

    pub fn new_at(x: f64, y: f64) -> Car {
        let mut default_car = Car::default();
        default_car.x = x; 
        default_car.y = y;

        default_car.hitbox.translate(x, y);
        default_car.polygons.iter_mut().for_each(|p| p.translate(x, y));

        default_car 
    }

    pub fn accelerate(&mut self) {
        self.speed += self.acceleration;
        
    }

    pub fn decelerate(&mut self) {
        self.speed -= self.acceleration;
    }

    pub fn brake(&mut self) {
        if Math::abs(self.speed) > 0.0 {
            self.decelerate();
        } else {
            self.speed = 0.0;
        }
    }

    pub fn turn_left(&mut self) {
        self.angle -= 0.1;
        self.hitbox.rotate(-0.1);
        self.polygons.iter_mut().for_each(|p| p.rotate(-0.1));
    }

    pub fn turn_right(&mut self) {
        self.angle += 0.1;
        self.hitbox.rotate(0.1);
        self.polygons.iter_mut().for_each(|p| p.rotate(0.1));
    }

    pub fn limit_speed(&mut self) {
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        } else if self.speed < -self.max_speed {
            self.speed = -self.max_speed;
        } else if Math::abs(self.speed) < Math::abs(self.friction) {
            self.speed = 0.0;
        } else if self.speed > 0.0 {
            self.speed -= self.friction;
        } else if self.speed < 0.0 {
            self.speed += self.friction;
        }
    }

    pub fn move_coords(&mut self) {
        if self.speed == 0.0 { return; }

        let d_x = self.speed * self.angle.cos();
        let d_y = self.speed * self.angle.sin();

        self.x += d_x;
        self.y += d_y;

        self.hitbox.translate(d_x, d_y);
        self.polygons.iter_mut().for_each(|p| p.translate(d_x, d_y));
    }

    pub fn apply_controls(&mut self) {
        if self.controls.forward { self.accelerate(); }
        if self.controls.backward { self.decelerate(); }
        if self.controls.left { self.turn_left(); }
        if self.controls.right { self.turn_left(); }
        if self.controls.brake { self.brake(); }

        self.limit_speed();
    }

    
    pub fn update(&mut self) {
        self.apply_controls();
        self.move_coords();
    }

    pub fn generate_polygons(width: f64, height: f64, angle: f64) -> Vec<Polygon> {
        let mut polygons = Vec::with_capacity(1);

        let car_box = Polygon::new(vec![
            Point::default(),   
            Point::new(0.0, 50.0),
            Point::new(100.0, 50.0),
            Point::new(100.0, 0.0)
        ], "blue".to_owned());

        let mut car_tire_back = Polygon::rectangle(25.0, 0.0, 25.0, 25.0, 0.0);
        let mut car_tire_front = Polygon::rectangle(75.0, 0.0, 25.0, 25.0, 0.0);
        car_tire_front.fill_color = "gray".to_owned();
        car_tire_back.fill_color = "gray".to_owned();

        let mut car_front = Polygon::rectangle(85.0, 47.5, 30.0, 25.0, 0.0);
        car_front.fill_color = "yellow".to_owned();

        polygons.push(car_box);
        polygons.push(car_tire_back);
        polygons.push(car_tire_front);
        polygons.push(car_front);

        polygons.iter_mut().for_each(|p| {
            p.translate(-50.0, -25.0); 
            p.scale(100.0 / height, 50.0 / width); 
            p.rotate_origin(angle);
        });

        polygons
    }
}

impl Drawable for Car {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.hitbox.draw(context);
        // self.sensors.draw(context);
        self.polygons.iter().for_each(|p| p.draw(context));
    }
}

impl Default for Car{
    fn default() -> Car {
        let width = 50.0;
        let height = 100.0;
        let angle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;

        Car::new(
            0.0,
            0.0,
            width,
            height,
            angle,
            10.0,
            25.0,
            0.1,
            0.05,
        )
    }
}
