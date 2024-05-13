use crate::{geo::*, Road, Drawable};

pub struct FinishLine {
    pub start: Polygon,
    pub end: Polygon,
}

impl FinishLine {
    pub fn new(road: &Road) -> FinishLine {
        if road.lines.len() < 2 {
            return FinishLine {
                start: Polygon::new(vec![], "black".to_string()),
                end: Polygon::new(vec![], "black".to_string()),
            }
        }

        let start_segment = road.lines.first().unwrap();
        let end_segment = road.lines.last().unwrap();

        let start = FinishLine::create_polygon(
            &start_segment.left.start, 
            &start_segment.right.start, 

            &start_segment.left.end, 
            &start_segment.right.end, 
            "gold", 100.0
        ); 

        let end = FinishLine::create_polygon(
            &end_segment.left.end, 
            &end_segment.right.end, 

            &end_segment.left.start, 
            &end_segment.right.start, 
            "lime", 40.0
        ); 

        FinishLine {
            start, end
        }
    }

    pub fn create_polygon(a: &Point, b: &Point, c: &Point, d: &Point, color: &str, width: f64) -> Polygon {
        let mut end_a = a.clone();
        let mut end_b = b.clone();

        end_a.move_towards_point(c, width);
        end_b.move_towards_point(d, width);

        let polygon = Polygon::new(vec![a.clone(), b.clone(), end_b, end_a], color.to_owned());

        polygon
    }

    pub fn get_start_angle(&self) -> f64 {
        if self.start.points.len() < 4 {
            return 0.0;
        }

        let a = &self.start.points[0];
        let d = &self.start.points[3];

        a.angle(d)
    }
}

impl Drawable for FinishLine {
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.start.draw(context);
        self.end.draw(context);
    }
}
