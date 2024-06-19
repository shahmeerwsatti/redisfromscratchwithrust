use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
println!("Guess the number: ");

let mut guessednum = String::new();

let secretnum: u32 = rand::thread_rng().gen_range(1..=100_);

io::stdin().read_line(&mut guessednum).expect("Failed to read line.");

println!("You guessed: {guessednum}");

let guessednum: u32 = guessednum.trim().parse().expect("Please type a number!");

match guessednum.cmp(&secretnum)
{
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
}
