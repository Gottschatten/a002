/*use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    stdout.write_all(b"clear")?;

    Ok(())
}*/

use std::io;
use std::process::Command;

fn main() {
    println!("Test");
    Command::new("clear").status().unwrap();
    println!("Test2");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
