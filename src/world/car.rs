use web_sys::js_sys::Math;

use crate::{
    Drawable, 
    geo::{Polygon, Point}, 
    sprite, console_log, sensors::Sensors
};

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

    pub sensors: Sensors,    
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

        let sensors = Sensors::new(x, y, 6, 200.0, std::f64::consts::PI / 2.0, angle);

        Car {
            x, y, width, height, angle, 
            speed, max_speed, acceleration, friction, 
            polygons, controls, hitbox,
            sensors,
            has_collided: false,
        }
    } 

    pub fn new_at(x: f64, y: f64) -> Car {
        let mut default_car = Car::default();
        default_car.x = x; 
        default_car.y = y;

        default_car.hitbox.translate(x, y);
        default_car.polygons.iter_mut().for_each(|p| p.translate(x, y));
        default_car.sensors.translate(x, y);

        default_car 
    }

    pub fn accelerate(&mut self) {
        self.speed += self.acceleration;
        
    }

    pub fn decelerate(&mut self) {
        self.speed -= self.acceleration;
    }

    pub fn brake(&mut self) {
        let brake_force = self.acceleration * 2.5;
        let new_speed = Math::max(Math::abs(self.speed) - brake_force, 0.0);

        if self.speed >= 0.0 {
            self.speed = new_speed;
        } else {
            self.speed = -new_speed;
        } 
    }

    pub fn turn(&mut self, d_angle: f64) {
        let d_angle = if self.speed >= 0.0 { d_angle } else { -d_angle }; 
        self.angle += d_angle;
        self.hitbox.rotate(d_angle);

        let car_origin = Point::new(self.x, self.y);
        self.polygons.iter_mut().for_each(|p| p.rotate_around(d_angle, &car_origin));
        self.sensors.rotate(d_angle);
    }

    pub fn turn_left(&mut self) {
        self.turn(-0.1);
    }

    pub fn turn_right(&mut self) {
        self.turn(0.1);
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
        self.sensors.translate(d_x, d_y);
    }

    pub fn apply_controls(&mut self) {
        if self.controls.forward { self.accelerate(); }
        if self.controls.backward { self.decelerate(); }
        if self.controls.left { self.turn_left(); }
        if self.controls.right { self.turn_right(); }
        if self.controls.brake { self.brake(); }

        self.limit_speed();
    }

    
    pub fn update(&mut self) {
        self.apply_controls();
        self.move_coords();
    }

    pub fn generate_polygons(width: f64, height: f64, angle: f64) -> Vec<Polygon> {
        let mut sprite = sprite::get_car_sprite(); 

        sprite.iter_mut().for_each(|poly| {
            poly.translate(-50.0, -25.0); 
            poly.scale_origin(height / 100.0, width / 50.0); 
            poly.rotate_origin(angle);
        });

        sprite
    }
}

impl Drawable for Car {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        // self.hitbox.draw(context);
        self.sensors.draw(context);
        self.polygons.draw(context);
    }
}

impl Default for Car{
    fn default() -> Car {
        let width = 40.0;
        let height = 80.0;

        Car::new(
            0.0,
            0.0,
            width,
            height,
            0.0,
            0.0,
            20.0,
            0.1,
            0.05,
        )
    }
}
