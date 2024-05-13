use crate::{world::*, Drawable};
use self::player::Player;

pub struct Entities {
    pub cars: Vec<Car>,
    pub player: Option<Player>,
    pub road: Road,
    pub finish_line: FinishLine,
}

impl Entities {
    pub fn new() -> Entities {
        let mut cars = Vec::new();
        let road = Road::load();
        let player = Player::new();
        let finish_line = FinishLine::new(&road);

        let start_origin = finish_line.start.center();
        let start_angle = finish_line.start.points[0].angle(&finish_line.start.points[3]);
        for _ in 0..10 {
            let mut car = Car::new_at(start_origin.x, start_origin.y);
            car.turn(start_angle);
            cars.push(car)
        }

        Entities {
            cars,
            player: Some(player),
            road,
            finish_line
        }
    }

    pub fn update(&mut self) {
        self.cars.iter_mut().for_each(|c| c.update());
        if let Some(player) = &self.player {
            player.update();
        }
        // todo!(); 
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.road.draw(context);
        self.finish_line.draw(context);  

        self.cars.draw(context);
        if let Some(player) = &self.player {
            player.draw(context);
        }
    }
}

