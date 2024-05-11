use crate::helpers::lerpf;

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
        let a = self.end.y - self.start.y;
        let b = self.start.x - self.end.x;
        let c = a * self.start.x + b * self.start.y;

        let u = other.start.x * a + other.start.y * b - c;
        let v = other.end.x * a + other.end.y * b - c;

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
