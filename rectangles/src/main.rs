#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width ^ self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // tuple approch
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect1));

    // struct approch
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(&rect2)
    );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);


    // now struct with methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    // additional methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    // associated functions
    let square = Rectangle::square(5);

}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area1(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}