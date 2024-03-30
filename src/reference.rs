fn main(){
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}", s1, len);
    
    let mutable = mutable_reference(&mut s1);

    println!("s1 is now {}", mutable);
}
// The '&' is a reference, and they allow you to refer to some value without taking ownership of it.
fn calculate_length(s: &mut String) -> usize{
    s.len()
}

fn mutable_reference(s: &mut String) -> &mut String{
    s.push_str(" world");
    s
}