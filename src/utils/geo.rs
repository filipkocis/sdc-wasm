use wasm_bindgen::JsValue;

use crate::{helpers::lerpf, console_log};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn angle(&self, other: &Point) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }

    pub fn move_towards(&mut self, angle: f64, distance: f64) {
        self.x = self.x + angle.cos() * distance;
        self.y = self.y + angle.sin() * distance;
    }

    pub fn move_towards_point(&mut self, other: &Point, distance: f64) {
        let angle = self.angle(other);
        self.move_towards(angle, distance)
    }

    pub fn move_away(&mut self, angle: f64, distance: f64) {
        self.move_towards(angle, -distance)
    }

    pub fn move_away_point(&mut self, other: &Point, distance: f64) {
        self.move_towards_point(other, -distance)
    }

    pub fn from_event(event: &web_sys::MouseEvent, element: &web_sys::Element) -> Point {
        let rect = element.get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();

        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub point: Point,
    pub offset: f64,
    pub intersects: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }

    pub fn angle(&self) -> f64 {
        self.start.angle(&self.end)
    }

    pub fn move_towards(&mut self, angle: f64, distance: f64) {
        self.start.move_towards(angle, distance);
        self.end.move_towards(angle, distance);
    }

    pub fn move_away(&mut self, angle: f64, distance: f64) {
        self.move_towards(angle, -distance);
    }

    pub fn move_towards_separate(&mut self, angle: f64, distance_start: f64, distance_end: f64) {
        self.start.move_towards(angle, distance_start);
        self.end.move_towards(angle, distance_end);
    }

    pub fn move_away_separate(&mut self, angle: f64, distance_start: f64, distance_end: f64) {
        self.move_towards_separate(angle, -distance_start, -distance_end);
    }

    pub fn intersects(&self, other: &Line) -> bool {
        Line::intersects_lines(&self.start, &self.end, &other.start, &other.end)
        // let a = self.end.y - self.start.y;
        // let b = self.start.x - self.end.x;
        // let c = a * self.start.x + b * self.start.y;
        //
        // let u = other.start.x * a + other.start.y * b - c;
        // let v = other.end.x * a + other.end.y * b - c;
        //
        // u * v <= 0.0
    }

    pub fn check_aabb_intersection(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
        let min_x_1 = a.x.min(b.x);  
        let max_x_1 = a.x.max(b.x);
        let min_y_1 = a.y.min(b.y);  
        let max_y_1 = a.y.max(b.y);

        let min_x_2 = c.x.min(d.x);  
        let max_x_2 = c.x.max(d.x);
        let min_y_2 = c.y.min(d.y);  
        let max_y_2 = c.y.max(d.y);

        return !(
            max_x_1 < min_x_2 ||
            max_y_1 < min_y_2 ||
            max_x_2 < min_x_1 ||
            max_y_2 < min_y_1
        );
    }

    pub fn intersects_lines(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
        let d_y = b.y - a.y;
        let d_x = a.x - b.x;
        let m = d_y * a.x + d_x * a.y;

        let u = c.x * d_y + c.y * d_x - m;
        let v = d.x * d_y + d.y * d_x - m;

        u * v <= 0.0
    }

    pub fn get_intersection(&self, other: &Line) -> Option<Intersection> {
        let a = &self.start;
        let b = &self.end;
        let c = &other.start;
        let d = &other.end;

        let denominator = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);

        if denominator == 0.0 {
            return None;
        }

        let t = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / denominator;
        let u = ((a.x - b.x) * (c.y - a.y) - (a.y - b.y) * (c.x - a.x)) / denominator;

        let intersects = t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0;

        let x = lerpf(a.x, b.x, t);
        let y = lerpf(a.y, b.y, t);
        let point = Point { x, y };

        Some(Intersection { point, offset: t, intersects })
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.begin_path();
        context.move_to(self.start.x, self.start.y);
        context.line_to(self.end.x, self.end.y);
        context.stroke();
    }
}

pub struct Polygon {
    pub points: Vec<Point>,
    pub fill_color: String,
    pub stroke_color: String,
}

impl Polygon {
    pub fn new(points: Vec<Point>, color: String) -> Polygon {
        Polygon { 
            points, 
            fill_color: color.to_owned(), 
            // stroke_color: color 
            stroke_color: "black".to_owned()
        }
    }
    
    pub fn center(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;

        for point in self.points.iter() {
            x += point.x;
            y += point.y;
        }

        Point { 
            x: x / self.points.len() as f64,
            y: y / self.points.len() as f64,
        }
    }

    pub fn rectangle(x: f64, y: f64, width: f64, height: f64, angle: f64) -> Polygon {
        let half_width = width / 2.0;
        let half_height = height / 2.0;


        let points = vec![
            Point::new(x - half_width, y - half_height),
            Point::new(x + half_width, y - half_height),
            Point::new(x + half_width, y + half_height),
            Point::new(x - half_width, y + half_height)
        ];

        let mut polygon = Polygon::new(points, "black".to_owned());
        polygon.rotate(angle);

        console_log!("{} {} : {:?}", x, y, polygon.points);

        polygon
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        if self.points.len() < 2 {
            return;
        }

        context.set_fill_style(&JsValue::from_str(self.fill_color.as_str()));
        context.set_stroke_style(&JsValue::from_str(self.stroke_color.as_str()));
        context.begin_path();

        context.move_to(self.points[0].x, self.points[0].y);
        for point in self.points.iter().skip(1) {
            context.line_to(point.x, point.y);
        }

        context.close_path();
        context.fill();
        context.stroke();
    }

    pub fn intersects(&self, other: &Polygon) -> bool {
        for i_self in 0..self.points.len() {
            let j_self = (i_self + 1) % self.points.len();

            let a = &self.points[i_self];
            let b = &self.points[j_self];

            for i_other in 0..other.points.len() {
                let j_other = (i_other + 1) % other.points.len();
                
                let c = &other.points[i_other];
                let d = &other.points[j_other];

                if Line::check_aabb_intersection(a, b, c, d) {
                    continue;
                }

                if Line::intersects_lines(a, b, c, d) {
                    return true;
                }
            }
        }

        false
    }

    pub fn rotate(&mut self, angle: f64) {
        let center = self.center();
        self.rotate_around(angle, &center)
    }

    pub fn rotate_origin(&mut self, angle: f64) {
        let center = Point::default();
        self.rotate_around(angle, &center);
    }

    pub fn rotate_around(&mut self, angle: f64, center: &Point) {
        let cos = angle.cos();
        let sin = angle.sin();

        for point in self.points.iter_mut() {
            let x = point.x - center.x;
            let y = point.y - center.y;

            point.x = center.x + x * cos - y * sin;
            point.y = center.y + x * sin + y * cos;
        }
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        for point in self.points.iter_mut() {
            point.x += x;
            point.y += y;
        }
    }

    pub fn scale(&mut self, factor_x: f64, factor_y: f64) {
        let center = self.center();

        for point in self.points.iter_mut() {
            point.x = center.x + (point.x - center.x) * factor_x;
            point.y = center.y + (point.y - center.y) * factor_y;
        }
    }

    pub fn scale_origin(&mut self, factor_x: f64, factor_y: f64) {
        for point in self.points.iter_mut() {
            point.x *= factor_x;
            point.y *= factor_y;
        }
    }

    pub fn move_towards(&mut self, angle: f64, distance: f64) {
        for point in self.points.iter_mut() {
            point.move_towards(angle, distance)
        }
    }
}
