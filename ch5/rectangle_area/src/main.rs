#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

enum CanHold {
    Yes,
    Rotate,
    No,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_hold_or_rotate(&self, other: &Rectangle) -> CanHold {
        if self.width > other.width && self.height > other.height {
            return CanHold::Yes;
        } else if self.height > other.width && self.width > other.height {
            return CanHold::Rotate;
        }
        CanHold::No
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // Method one
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(width1, height1),
    );

    // Improve with tuple
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect),
    );

    // Improve with struct
    let rect = Rectangle {width: 30, height: 50};
    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect),
    );

    // Improve with method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area(),
    );

    // Display trait
    println!("rect: {:#?}", rect);

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 20,
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold rect3: {}", rect.can_hold(&rect3));

    for other in [rect1, rect2, rect3] {
        match rect.can_hold_or_rotate(&other) {
            CanHold::Yes => println!("rect can hold"),
            CanHold::Rotate => println!("rect can hold if rotated"),
            CanHold::No => println!("rect cannot hold"),
        }
    }

    let rect4 = Rectangle::square(30);

    println!("rect can hold rect4: {}", rect.can_hold(&rect4));
}

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
