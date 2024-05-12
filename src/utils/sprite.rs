use crate::geo::{Polygon, Point};

pub fn get_car_sprite() -> Vec<Polygon> {
    let mut polygons = Vec::with_capacity(1);

    let car_box = Polygon::new(vec![
        Point::default(),   
        Point::new(0.0, 50.0),
        Point::new(100.0, 50.0),
        Point::new(100.0, 0.0)
    ], "blue".to_owned());

    let car_tire_back_left = Polygon::rectangle(20.0, 0.0, 25.0, 20.0, 0.0);
    let car_tire_back_right = Polygon::rectangle(20.0, 50.0, 25.0, 20.0, 0.0);
    let car_tire_front_left = Polygon::rectangle(80.0, 0.0, 25.0, 20.0, 0.0);
    let car_tire_front_right = Polygon::rectangle(80.0, 50.0, 25.0, 20.0, 0.0);

    let mut car_front = Polygon::rectangle(85.0, 25.0, 30.0, 40.0, 0.0);
    car_front.fill_color = "yellow".to_owned();

    polygons.push(car_tire_back_left);
    polygons.push(car_tire_back_right);
    polygons.push(car_tire_front_left);
    polygons.push(car_tire_front_right);
    polygons.push(car_box);
    polygons.push(car_front);
   
    polygons
}

pub fn get_road_sprite() -> Vec<Polygon> {
    todo!()
}
