use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::{self, Write};

struct GameData {
    random_number: u32,
    guess: u32,
    round_counter: u32,
}

fn main() {
    let mut game_data = GameData {
        random_number: thread_rng().gen_range(1..=100),
        guess: 0,
        round_counter: 0,
    };
    loop {
        game_data.round_counter = game_data.round_counter + 1;
        let mut guess = String::new();

        print!("Enter a number between 1 and 100: ");
        io::stdout().flush().expect("Ooops");
        io::stdin().read_line(&mut guess).expect("Oops!");

        game_data.guess = match guess.trim().parse() {
            Ok(nr) => nr,
            Err(_) => continue,
        };

        println!("Your guess: {0}", game_data.guess);

        match game_data.guess.cmp(&game_data.random_number) {
            Ordering::Less => println!("Uh oh, too low!"),
            Ordering::Greater => println!("Uh oh, too high!"),
            Ordering::Equal => {
                println!(
                    "Grrrrrreat! you got it! It took you {0} rounds.",
                    game_data.round_counter
                );
                break;
            }
        }
    }
}
