fn main(){
    let s1 = String::from("Hello");
    //ownership transfer
        // s1 = ownership_transfer(s1);
        // println!("{}",s1);

    //borrowing
    borrow_var(&s1);
    println!("{}",s1);
}
fn borrow_var(some_string : &String) ->() {
    println!("{}",some_string);
}

#[allow(dead_code)] 
fn ownership_transfer(some_string : String)-> String{
    println!("{}",some_string);

    return some_string;
}