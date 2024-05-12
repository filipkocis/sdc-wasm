use rand::Rng;

use crate::{world::*, Drawable};

pub struct Entities {
    pub cars: Vec<Car>,
    // pub player: Option<Player>,
    pub road: Road,
    // pub finish_line: FinishLine,
}

impl Entities {
    pub fn new() -> Entities {
        let mut rng = rand::thread_rng();
        let mut cars = Vec::new();
        let road = Road::new();

        for _ in 0..1_000 {
            cars.push(Car::new_at(rng.gen_range(0.0..500.0), 350.0))
            // cars.push(Car::new_at(rng.gen_range(500.0..800.0), 600.0))
        }

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

