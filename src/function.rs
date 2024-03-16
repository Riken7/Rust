fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 { //return type set to i32
    x + 1 //no semicolon, so it is an expression
}