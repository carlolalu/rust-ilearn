use std::ops::Add;

struct Point {
x: f64,
y: f64,
}

//  the trait is:

//  trait Add<Rhs=Self(=DefaultType)> {
//  type Output;
//
//  fn add(&self, other: Rhs) -> Self::Output;
//  }

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        return Point{x: self.x + other.x, y: self.y + other.y};
    }
}


// for another type is still implementable
impl Add<f64> for Point {
    type Output = Point;
    fn add(self, other: f64) -> Self::Output {
        return Point{x: self.x + other, y: self.y + other};
    }
}

// the orphan rule here is somehow still holding if we think of the generic parameter as another parent
impl Add<Point> for f64 {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        return Point{x: self + other.x, y: self + other.y};
    }
}


// what about the fmt::Display trait? I could read more about this on the input output chapter (ch18 from the orelly)

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// I would claim that this is really strange. What is a formatter? Why do I have amacro which prints but also does not end wiht a semicolon, i..e returns something like 'fmt::Result'?



fn main() {
    println!("Hello, world!");
}
