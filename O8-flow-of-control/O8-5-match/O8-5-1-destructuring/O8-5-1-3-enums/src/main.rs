#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32),
}
fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("what color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::RGB(r,g,b) => 
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        _ => println!("a color is a color"),
    }
}
