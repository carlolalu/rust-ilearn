
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(self:&Self) -> u32 {
        return self.width*self.height;
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
    fn square(side:u32) -> Rectangle {
        return Rectangle{ width: side, height: side};
    }
}

fn main() {
    let rectangle1 = Rectangle{ 
        width: 30,
        height: 50,
        };

    let area1 = calculate_area_rectangle(&rectangle1);

    println!("Hello, world! The area of {:?} is {}", rectangle1, area1);

    let area2 = rectangle1.area();

    println!("Hello, the area of {:?} is {} as calculated with the method for it", rectangle1, area2);

    let rectangle2 = Rectangle{
        width: 20,
        height: 30,
    };

    println!("{:?} can hold {:?}? {}", rectangle1, rectangle2, rectangle1.can_hold(&rectangle2));

    println!("{:?} can hold {:?}? {}", rectangle2, rectangle1, rectangle2.can_hold(&rectangle1));

    let square1 = Rectangle::square(40);

    println!("And our square is {:?}", square1);

}

fn calculate_area_rectangle(rectangle : &Rectangle) -> u32{
    let area = rectangle.width * rectangle.height;
    area
}
