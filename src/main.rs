
struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

struct Color(u32, u32, u32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Hackerutz123"),
        email: String::from("alexandru.i.mot@gmail.com"),
        sign_in_count: 1,
        
    };

    user1.email = String::from("hackerutz21@gmail.com");
    println!("user1: {}", user1.email);

    println!("Next up we have tuple structs: ");

    let blue = Color(0, 0, 255);

    println!("Color blue: {}, {}, {}", blue.0, blue.1, blue.2);
    

}