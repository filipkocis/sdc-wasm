use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

use crate::*;
use crate::geo::Point;

pub struct Game {
    pub entities: Rc<RefCell<Entities>>,
    pub network: Network,
    pub game_canvas: Rc<RefCell<Canvas>>,
    pub node_canvas: Rc<RefCell<Canvas>>,
}

impl Game {
    pub fn new() -> Game {
        let game_canvas = Rc::new(RefCell::new(Canvas::from("gameCanvas")));
        let node_canvas = Rc::new(RefCell::new(Canvas::from("nodeCanvas")));

        let entities = Rc::new(RefCell::new(Entities::new()));
        let network = Network { layers: vec![] };

        Game {
            entities,
            network,
            game_canvas,
            node_canvas,
        }
    }

    pub fn update(&self) {
        self.entities.borrow_mut().update();
        // self.network.update();
    }

    pub fn draw(&self) {
        let game_canvas = self.game_canvas.borrow();
        let node_canvas = self.node_canvas.borrow();

        game_canvas.resize();
        node_canvas.resize();

        game_canvas.clear();
        node_canvas.clear();

        self.entities.borrow().draw(&game_canvas.context);
    }

    pub fn run(&self) {
        self.update();
        self.draw();
    }

    pub fn start(self) {
        let animation_callback = Rc::new(RefCell::new(None));
        let callback_initializer = animation_callback.clone();

        {
            let context = self.game_canvas.clone();
            let entities = self.entities.clone();
            let mut rng = rand::thread_rng();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let point = Point::from_event(&event, &context.borrow().element);
                let entities_borrow_road = &mut entities.borrow_mut().road;

                entities_borrow_road.add_point(point, rng.gen_range(35.0..75.00));
                entities_borrow_road.construct();

                console_log!("{} {}", event.client_x(), event.client_y());
            });

            let _ = self.game_canvas.borrow().element.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
            // let f = || self.game_canvas.borrow().element.remove_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());

            closure.forget();
        }
        //
        // let cars = Rc::new(RefCell::new(Vec::new()));
        // for _ in 0..10000 {
        //     cars.borrow_mut().push(Car::default())
        // }
        //
        // let game_canvas = self.game_canvas.clone();
        // let node_canvas = self.game_canvas.clone();

        let mut i = 0;
        *callback_initializer.borrow_mut() = Some(Closure::new(move || {
            self.run();

            i += 1;
            let text = format!("requestAnimationFrame has been called {} times.", i);
            // js::get_element_by_id("rustOutput").set_text_content(Some(&text));

            js::request_animation_frame(animation_callback.borrow().as_ref().unwrap());
        }));

        js::request_animation_frame(callback_initializer.borrow().as_ref().unwrap());
    }
}
