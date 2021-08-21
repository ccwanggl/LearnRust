const MAX_SIZE:i32 = 100_000;

fn shadowing()
{
     // The variable default is immutiable
     let spaces = "    ";
     println!{"Shadowing: the \"spaces\" value is {}", spaces};

     let spaces = spaces.len();
     println!{"Shadowing: the \"spaces\" value is {}", spaces};
}

fn main() {

    let x = 8;
    println!("The value of x equals to: {}", x);

    let x = 10;
    println!("The value of x equals to: {}", x);

    let x = 100;
    println!("The value of x equals to: {}", x);

    shadowing();

    let mut spaces = "    ";
    // Err, we are not allowed to mutate a variable's type:
    //spaces = spaces.len();   
}
