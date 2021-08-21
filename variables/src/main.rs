mod value_tour;

const MAX_SIZE:i32 = 100_000;

fn main() {

    let x = 8;
    println!("The value of x equals to: {}", x);

    let x = 10;
    println!("The value of x equals to: {}", x);

    let x = 100;
    println!("The value of x equals to: {}", x);

    value_tour::value_tour();
    value_tour::shadowing();

    let mut spaces = "    ";
}
