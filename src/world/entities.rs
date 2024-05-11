use crate::{world::*, Drawable};

pub struct Entities {
    pub cars: Vec<Car>,
    // pub player: Option<Player>,
    pub road: Road,
    // pub finish_line: FinishLine,
}

impl Entities {
    pub fn new() -> Entities {
        let mut cars = Vec::new();
        for _ in 0..1_000 {
            cars.push(Car::default())
        }

        let road = Road::new();

        Entities {
            cars,
            road
        }
    }

    pub fn update(&mut self) {
        self.cars.iter_mut().for_each(|c| c.update());
        // todo!(); 
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.cars.iter().for_each(|c| c.draw(context));
        self.road.draw(context);
        // todo!(); 
    }
}

