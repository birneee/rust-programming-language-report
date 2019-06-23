fn main() {
    use core::f32::consts::PI;

    trait Shape{
        fn area(&self) -> f32;
    }

    struct Rectangle {
        width: f32,
        height: f32
    }

    struct Circle{
        radius: f32
    }

    impl Shape for Rectangle {
        fn area(&self) -> f32{
            self.width * self.height
        }
    }

    impl Shape for Circle{
        fn area(&self) -> f32{
            PI * self.radius * self.radius
        }
    }

    //let mut v : Vec<Shape> = Vec::new();

    let mut v: Vec<Box<Shape>> = Vec::new();
    v.push(Box::new(Rectangle{width: 23.0, height: 12.0}));
    v.push(Box::new(Circle{radius: 14.0}));

    for shape in v{
        println!("{}", shape.area());
    }
}