fn main() {
    println!("Rectangle function test using different approaches.");
    println!("This implementation is a bit dificult and unintuitive to use because in tuple we have to access fields by index.");
    
    
    let rectangle = (30, 50);


    println!("The area of {}x{} rectangle is {}.", rectangle.0, rectangle.1, area(rectangle));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}