struct User {
    active: bool,
    name: String,
    email: String,
}
struct Single; //unit like struct
fn call_user( name: String, email: String) -> User{
    User{
        active: true,
        name,
        email,
    }
}
fn main(){
    let user1 = call_user(String::from("usr1"), String::from("usr1@gmail.com"));
    // let user2 = call_user(String::from("usr2"), String::from("usr2@gmail.com"));
    let user2 = user1;

    //since the value is borrowed, it can't be used again in the next line.. 
    //user1.name is no longer valid and will throw an error
    println!("{} and {} are friends", user1.name, user2.name); 
}