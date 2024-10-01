trait Shape {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

struct ShapeFactory;

impl ShapeFactory {
    fn create_shape(shape_type: &str) -> Box<dyn Shape> {
        match shape_type {
            "Circle" => Box::new(Circle),
            "Square" => Box::new(Square),
            _ => panic!("Unknown shape type!"),
        }
    }
}


#[cfg(test)]
mod factory_test {
    use super::*;


    #[test]
    fn test_01() {
        let circle = ShapeFactory::create_shape("Circle");
        circle.draw();

        let square = ShapeFactory::create_shape("Square");
        square.draw();
    }
}

