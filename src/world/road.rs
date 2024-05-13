use wasm_bindgen::JsValue;

use crate::{geo::*, console_log, js, Drawable};


pub struct Road {
    pub lines: Vec<RoadLine>,
    pub points: Vec<RoadPoint>,
}

pub struct RoadPoint {
    pub point: Point,
    pub width: f64,
}

pub struct RoadLine {
    pub line: Line,
    pub left: Line,
    pub right: Line,
    pub start_width: f64,
    pub end_width: f64,
}

impl RoadLine {
    pub fn new(line: Line, start_width: f64, end_width: f64) -> RoadLine {
        let angle = line.angle();
        let angle = angle - 90.0f64.to_radians();

        let mut left = line.clone();
        let mut right = line.clone(); 

        left.move_towards_separate(angle, start_width, end_width);
        right.move_away_separate(angle, start_width, end_width);

        RoadLine {
            line, left, right, start_width, end_width
        }
    }

    pub fn from_points(a: &Point, b: &Point, start_width: f64, end_width: f64) -> RoadLine {
        let line = Line::new(a.clone(), b.clone());
        RoadLine::new(line, start_width, end_width)
    }

    pub fn from_road_points(a: &RoadPoint, b: &RoadPoint) -> RoadLine {
        let line = Line::new(a.point.clone(), b.point.clone());
        RoadLine::new(line, a.width, b.width)
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.save();
        context.set_line_width(5.0);
        context.set_fill_style(&JsValue::from_str("white"));

        context.set_stroke_style(&JsValue::from_str("black"));
        self.left.draw(context);
        self.right.draw(context);

        context.set_stroke_style(&JsValue::from_str("gray"));
        self.line.draw(context);

        context.fill();
        context.restore();
    }
}


impl Road {
    pub fn new() -> Road {
        Road {
            lines: vec![],
            points: vec![],
        }
    }

    pub fn add_point(&mut self, point: Point, width: f64) {
        self.points.push(RoadPoint {
            point,
            width,
        });
    }

    pub fn add_line(&mut self, line: RoadLine) {
        self.lines.push(line);
    }

    pub fn construct(&mut self) {
        self.lines.clear();
        for i in 0..self.points.len() {
            let a = &self.points[i];
            let b = &self.points[(i + 1) % self.points.len()];

            if i + 1 == self.points.len() {
                break;
            }

            self.add_line(RoadLine::from_road_points(a, b));
        }

        for i in 0..self.lines.len() {
            let lines = &mut self.lines;
            let next_index = i + 1; 

            if next_index % lines.len() == 0 && lines.len() > 1 {
                let connected_line = RoadLine::from_points(
                    &lines[i].line.end,     
                    &lines[0].line.start,
                    lines[i].end_width,
                    lines[0].start_width,
                );
                lines.push(connected_line);

                Road::merge(lines, next_index);
            }

            Road::merge(lines, i);
        }

        let points_str = self.points.iter().map(|p| format!("{:.2} {:.2} {:.2}", p.point.x, p.point.y, p.width)).collect::<Vec<String>>().join(", \n");

        js::get_element_by_id("rustOutput").set_text_content(Some(&points_str));
    }

    pub fn merge(lines: &mut [RoadLine], index: usize) {
        let next_index = (index + 1) % lines.len(); 

        let left_intersection = lines[index].left.get_intersection(&lines[next_index].left);
        let right_intersection = lines[index].right.get_intersection(&lines[next_index].right);

        if let Some(intersection) = left_intersection {
            lines[index].left.end = intersection.point; 
            lines[next_index].left.start = intersection.point; 
        }

        if let Some(intersection) = right_intersection {
            lines[index].right.end = intersection.point; 
            lines[next_index].right.start = intersection.point; 
        }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.lines.iter().for_each(|l| l.draw(context));
    }
}

impl Road {
    pub fn load() -> Road {
        let points = vec![
            RoadPoint { point: Point::new(191.0, 653.0), width: 53.19 }, 
            RoadPoint { point: Point::new(484.0, 715.0), width: 51.37 }, 
            RoadPoint { point: Point::new(750.0, 730.0), width: 71.04 }, 
            RoadPoint { point: Point::new(946.0, 671.0), width: 44.13 }, 
            RoadPoint { point: Point::new(1004.0, 551.0), width: 68.13 }, 
            RoadPoint { point: Point::new(969.0, 367.0), width: 68.58 }, 
            RoadPoint { point: Point::new(931.0, 174.0), width: 60.45 }, 
            RoadPoint { point: Point::new(799.0, 55.0), width: 62.85 }, 
            RoadPoint { point: Point::new(612.0, 97.0), width: 73.47 }, 
            RoadPoint { point: Point::new(509.0, 173.0), width: 50.77 }, 
            RoadPoint { point: Point::new(497.0, 300.0), width: 66.28 }, 
            RoadPoint { point: Point::new(470.0, 358.0), width: 46.56 }, 
            RoadPoint { point: Point::new(313.0, 278.0), width: 66.98 }, 
            RoadPoint { point: Point::new(153.0, 180.0), width: 72.08 }, 
            RoadPoint { point: Point::new(96.0, 320.0), width: 74.9 }, 
            RoadPoint { point: Point::new(89.0, 510.0), width: 43.34 }, 
        ];

        let mut road = Road { points, lines: vec![] };
        road.construct();

        road
    }
}
