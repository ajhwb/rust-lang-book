use std::io;

// Only work at maximum usize value
fn fibonacci(index: usize) -> usize {
    let mut x: usize = 0;
    let mut f: usize = 0;
    let mut a: usize = 0;
    let mut b: usize = 0;

    while x < index {
        if x == 0 {
            f = 0;
            a = f;
        } else if x == 1 {
            f = 1;
            b = f;
        } else {
            f = a + b;
            a = b;
            b = f;
        }
        x += 1;
    }
    f
}

fn main() {
    let mut index = String::new();

    println!("Enter Fibonacci #number: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Invalid index");
    if index <= 0 {
        println!("#number must be greater than 0!");
        return;
    }
    let number = fibonacci(index);
    println!("Fibonacci number #{} is {}", index, number);
}