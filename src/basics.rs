use std::io; 
fn main() {
    //checkout notion notes!
    println!("Hello, world!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let x = 5 ; 
    let y = 12 ;

    println!("The value of x is {x} and the value of x+y is {}", x+y);  
}