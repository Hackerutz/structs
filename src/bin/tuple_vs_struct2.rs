
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Rectangle function test using different approaches.");
    println!("This example has more clarity from using the struct and accessing fields by name instead of index.");
    

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


    println!("The area of {}x{} rectangle is {}.", rect1.width, rect1.height, area(&rect1));
    println!("the struct {rect1:?}");
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}