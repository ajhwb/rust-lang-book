fn main() {
    const DAYS: usize = 12;
    const NUMBERS: [&str; DAYS] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    const TURTLE_DOVES: &str = "Two turtle doves and a partridge in a pear tree";

    for day in 0..DAYS {
        println!("{}", day);
    }
}