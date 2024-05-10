use crate::helpers::lerpf;

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

pub struct Intersection {
    pub point: Point,
    pub offset: f64,
    pub intersects: bool,
}

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

    pub fn intersects(&self, other: &Line) -> bool {
        let a = self.end.y - self.start.y;
        let b = self.start.x - self.end.x;
        let c = a * self.start.x + b * self.start.y;

        let u = other.start.x * a + other.start.y * b - c;
        let v = other.end.x * a + other.end.y * b - c;

        u * v <= 0.0
    }

    pub fn get_intersection(&self, other: &Line) -> Option<Intersection> {
        let a1 = self.end.y - self.start.y;
        let b1 = self.start.x - self.end.x;
        let c1 = a1 * self.start.x + b1 * self.start.y;

        let a2 = other.end.y - other.start.y;
        let b2 = other.start.x - other.end.x;
        let c2 = a2 * other.start.x + b2 * other.start.y;

        let determinant = a1 * b2 - a2 * b1;
        
        if determinant == 0.0 {
            return None;
        }

        let u = (b2 * c1 - b1 * c2) / determinant;
        let t = (a1 * c2 - a2 * c1) / determinant;
        let intersects = u >= 0.0 && u <= 1.0 && t >= 0.0 && t <= 1.0;

        let x = lerpf(self.start.x, self.end.x, t);
        let y = lerpf(self.start.y, self.end.y, t);
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
