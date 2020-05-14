enum Card {
    Heart,
    Spade,
    Ace,
    Club,
}

#[allow(dead_code)]
fn get_card() -> Card {
    Card::Heart
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Trans,
}

#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    address: &'a str,
    age: u8,
    sex: Option<Sex>,
}

impl<'a> std::fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tName: {}\n\tAddress: {}\n\tSex: {:?}\n\tAge: {}\n}}",
            self.name(),
            self.address(),
            self.sex(),
            self.age()
        )
    }
}

impl<'a> Person<'a> {
    pub fn new<'b: 'a>(name: &'b str, address: &'b str, age: u8, sex: Option<Sex>) -> Self {
        Self {
            name,
            address,
            age,
            sex,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn address(&self) -> &'a str {
        self.address
    }

    pub fn sex(&self) -> Option<Sex> {
        self.sex.clone()
    }
}
