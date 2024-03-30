use std::io;
fn main(){
    let mut var1 = String::new();
    io::stdin().read_line(&mut var1).expect("Failed to read line");
    let var1 : u32 = var1.trim().parse().unwrap();
    let var2 : u32= if var1 < 5  {
        5
    } else {
        6
    };

    if var2 == 5 {
        println!("var2 is 5");
    } else {
        println!("var2 is 6");
    }

}