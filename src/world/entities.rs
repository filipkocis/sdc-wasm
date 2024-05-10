use crate::{world::*, Drawable};

pub struct Entities {
    pub cars: Vec<Car>,
    // pub player: Option<Player>,
    // pub road: Road,
    // pub finish_line: FinishLine,
}

impl Entities {
    pub fn new() -> Entities {
        let mut cars = Vec::new();
        for _ in 0..1_000 {
            cars.push(Car::default())
        }

        Entities {
            cars
        }
    }

    pub fn update(&mut self) {
        self.cars.iter_mut().for_each(|c| c.update());
        // todo!(); 
    }

    pub fn draw(&mut self, context: &web_sys::CanvasRenderingContext2d) {
        self.cars.iter_mut().for_each(|c| c.draw(context));
        // todo!(); 
    }
}

