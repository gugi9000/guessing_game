extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write;

fn main() {
    println!("Gæt tallet!");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Det, ikke særligt, hemmelige nummer er: {}", secret);

    loop {
        print!("Skriv dit gæt: ");
        // Hvis du bruger print! skal skærmen manuelt flushes:
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Kunne ikke indlæse.");

        let guess: u32 = guess.trim().parse()
            .expect("Skriv et tal, tak.");

        println!("Du gættede på {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less    => println!("For lavt."),
            Ordering::Greater => println!("For højt."),
            Ordering::Equal   => println!("Du vandt!"),
        }
    }
}
