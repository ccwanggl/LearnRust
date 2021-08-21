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
}


pub fn shadowing()
{
     // The variable default is immutable
     let spaces = "    ";
     println!("Shadowing: the \"spaces\" value is {0}", spaces);

     let spaces = spaces.len();
     println!("Shadowing: the \"spaces\" value is {0}", spaces);
}