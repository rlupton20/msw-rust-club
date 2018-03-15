extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let num_dice: u8 = 4;

    println!("This is petals around the rose");

    loop {
        let dice: Vec<u8> = roll_dice(num_dice);
        let num_petals: u8 = petals(&dice);

        println!("Dice {:?}", dice);

        println!("Make a guess: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read from stdin");

        let guess: u8 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        match guess.cmp(&num_petals) {
            Ordering::Equal => {
                println!("Nice one!");
                break;
            },
            _ => println!("Nah. It's {}. Try again", num_petals)
        }
    }
}


fn die_roll() -> u8 {
    rand::thread_rng().gen_range(1,7)
}


fn roll_dice(number_of_dice: u8) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    for _ in 0..number_of_dice {
        ret.push(die_roll());
    }
    ret
}


fn petals(dice: &Vec<u8>) -> u8 {
    dice.iter()
        .map(|ds| match *ds {
            3 => 2,
            5 => 4,
            _ => 0
        })
        .fold(0, |acc, x| acc + x)
}
