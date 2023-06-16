extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    let secret = rand::thread_rng().gen_range(0..100);

    println!("Guessings a numero betweeno uno and uno hundred");
    println!("Please entero da numero");
    let mut guess = asko_numero();
    let mut flag: bool = false;
    let mut first: bool = true;





    while flag != true {
        if first == false{
            println!("Please entero anoda numero");
            guess = asko_numero();

        }
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Smallo"),
            Ordering::Greater => println!("Mucho Grande"),
            Ordering::Equal => {
                println!("Congrajewleshuns you does a winnings");
                flag = true; },
        }
    first=false;
    }

}


fn asko_numero () -> u32 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to readeros");
    let guess: u32 = guess.trim().parse()
        .expect("Donde entero non numero!");
    return guess;
}