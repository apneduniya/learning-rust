

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn main() {
    let rect1 = Rectangle{
        height: 32,
        width: 46
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}


fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}





