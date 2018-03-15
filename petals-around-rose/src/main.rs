extern crate rand;

use std::io;
use rand::Rng;


fn main() {
    let num_dice: u8 = 4;

    println!("This is petals around the rose");

    loop {
        let dice: Vec<u8> = roll_dice(num_dice);
        let num_petals: u8 = petals(&dice);

        println!("Dice {:?}", dice);

        println!("Make a guess: ");
        let guess: u8 = get_guess();

        if guess == num_petals {
            println!("Nice one");
            break;
        }
        else {
            println!("Nah. Tis {}. Try again.\n", num_petals);
        }
    }
}


fn get_guess() -> u8 {
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read from stdin");

    guess.trim().parse()
        .expect("Please type a number >0 and <256")
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
