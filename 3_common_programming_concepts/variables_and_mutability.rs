fn main() {
    // Variable declaration
    let a = 32;
    // Explicit type
    let b: bool = true;
    let c: i8 = 0x7f; // explicit type annotation
    let d = "Rusty";
    println!("a = {} b = {} c = {} d = {}", a, b, c, d);

    // Variables are immutable by default
    let x = 4;
    println!("x = {}", x);
    // x = 5; // Compile error, immutable variable assignment

    // Declare mutable variables
    let mut y = 14;
    println!("y = {}", y);
    y = 16;

    println!("y = {}", y);

    // constant
    // constant variable should has UPPERCASE name
    // const z: u32 = 51234125; // Compile time warning (non UPPERCASE name)
    const Z: u32 = 51231245;
    println!("Z = {}", Z);

    // Shadowing
    let s = 1; // First var
    let s = s + 1; // Shadow previous var by declaring new s
    let s = s * 2; // Shadow previous var by declaring new s
    println!("s = {}", s);
    // Shadow with new type
    let spaces = "   "; // str
    let spaces = spaces.len(); // int
    println("spaces={}", spaces);
}
