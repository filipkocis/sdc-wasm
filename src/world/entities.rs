use std::{cell::RefCell, rc::Rc};

use crate::{world::*, Drawable, console_log};
use self::player::Player;

pub struct Entities {
    pub cars: Rc<RefCell<Vec<Car>>>,
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
        let start_angle = finish_line.get_start_angle();
        for _ in 0..10 {
            let mut car = Car::new_at(start_origin.x, start_origin.y);
            car.turn(start_angle);
            cars.push(car)
        }

        Entities {
            cars: Rc::new(RefCell::new(cars)),
            player: Some(player),
            road,
            finish_line
        }
    }

    pub fn update(&mut self) {
        self.cars.borrow_mut().iter_mut().for_each(|c| c.update(&self.road));
        if let Some(player) = &self.player {
            player.update(&self.road);
        }
        // todo!(); 
        
        // self.check_collisions();
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.road.draw(context);
        self.finish_line.draw(context);  

        self.cars.borrow_mut().draw(context);
        if let Some(player) = &self.player {
            player.draw(context);
        }
    }

    // pub fn check_collisions(&mut self) {
    //     self.cars.borrow_mut().iter_mut().for_each(|car| {
    //         if car.hitbox.intersects(&self.road.hitbox) {
    //             car.collide();
    //         }
    //     });
    //
    //     if let Some(player) = &mut self.player {
    //         if player.car.borrow_mut().hitbox.intersects(&self.road.hitbox) {
    //             player.car.borrow_mut().collide();
    //             console_log!("intersecting");
    //         }
    //     }
    // }
}

