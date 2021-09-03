pub fn value_tour() {
    // let authenticated: bool = true;
    // if authenticated{
    //     todo!()
    // }
    // else {
    //     todo!()
    // }

    // modify value
    let mut total:i32 = 10;
    total += 1;
    println!("The total value is {0}", total);

    // pass parameter(s) to function (By value)
    let name: String = "Guoliang".to_string();
    print_my_name(name);

    // pass parameter(s) to function (By reference)
}

fn print_my_name(name: String)
{
    println!("Your name is {0}, from function parameter!", name);
}

pub fn shadowing()
{
     // The variable default is immutable
     let spaces = "    ";
     println!("Shadowing: the \"spaces\" value is {0}", spaces);

     let spaces = spaces.len();
     println!("Shadowing: the \"spaces\" value is {0}", spaces);
}