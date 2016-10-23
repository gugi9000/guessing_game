extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write;

fn main() {
    println!("Gæt tallet mellem 1 og 100!");

    let secret = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    // Gør det mindre sjovt:
    // println!("Det, ikke særligt, hemmelige nummer er: {}", secret);

    loop {
        print!("Skriv dit gæt: ");
        // Hvis du bruger print! skal skærmen manuelt flushes:
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Kunne ikke indlæse.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        tries = tries + 1;
        println!("I gæt nummer {} er {}", tries, guess);

        match guess.cmp(&secret) {
            Ordering::Less    => println!("For lavt."),
            Ordering::Greater => println!("For højt."),
            Ordering::Equal   => {
                println!("Du vandt på bare {} gæt!", tries);
                break;
            }
        }
    }
}
