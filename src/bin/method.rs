#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    println!("This is a program to test method specific to a struct.");
    
    

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("This rectangle ({}x{}) has area of {}.", rect1.width, rect1.height, rect1.area());
    
    
}