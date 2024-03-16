use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guessing game ! ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        let mut guess = String::new();
        println!("Please input your guess : " );
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("You quit the game !");
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number !");
                continue;
            }
        };
        
        println!("Your guess is : {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small !, guess a bigger number."),
            Ordering::Greater => println!("Too big !, guess a smaller number."),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
}

}