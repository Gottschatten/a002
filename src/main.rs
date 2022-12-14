use print::Kunden;
use std::io;
use std::process::Command;

mod print;
fn main() {
    Command::new("clear").status().unwrap();

    let kunden = Kunden::build("user");
    let kunden2 = Kunden::build("user1");
    let table = Kunden::table(kunden.m_vec(kunden2));
    println!("{}", table);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
