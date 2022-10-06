use print::{Kunden};


mod print;
fn main() {
    let kunden = Kunden::build("user");
    let kunden2 = Kunden::build("user1");
    let table = Kunden::table(kunden.m_vec(kunden2));
    println!("{}", table);
}
