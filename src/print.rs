use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct Kunden {
    id: u32,
    name: &'static str,
}

impl Kunden {
    pub fn build(name: &'static str) -> Kunden {
        let id = 0b1001;
        Kunden { id, name }
    }

    pub fn m_vec(self, _add: Self) -> Vec<Kunden> {
        vec![self, _add]
    }
    pub fn table(vect: Vec<Kunden>) -> String {
        Table::new(vect).to_string()
    }
}
