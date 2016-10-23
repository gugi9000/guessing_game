use std::io;

fn main() {
    println!("Gæt tallet!");
    print!("Skriv dit gæt: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Kunne ikke indlæse.");

    println!("Du gættede på {}", guess);
}
