fn main(){
    use core::ops::Add;
    use std::fmt;

    struct Vector2D{
        x: i32,
        y: i32
    }

    impl Add for Vector2D{
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl fmt::Display for Vector2D{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let a = Vector2D{x: 2, y: -1};
    let b = Vector2D{x: 4, y: 0};
    let c = a + b;

    println!("{}", c);
}