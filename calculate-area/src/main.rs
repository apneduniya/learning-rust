

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn is_bigger(&self, other_rect: &Rectangle) -> bool {
        self.area() > other_rect.area()
    }
}

impl Rectangle {
    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}


fn main() {
    let rect1 = Rectangle{
        height: 32,
        width: 46
    };

    let rect2 = Rectangle {
        height: 45,
        width: 10
    };

    println!("The area of the rect1 is {} square pixels.", rect1.area());
    println!("The area of the rect2 is {} square pixels.\n", rect2.area());

    println!("The dimensions of the rect1 is {}x{} pixels.", rect1.get_height(), rect1.get_width());
    if rect1.width() { println!("Is width of the rect1 non-zero? {}\n", rect1.width()); }

    println!("Is rect1 bigger than rect2? {}", rect1.is_bigger(&rect2))
}





