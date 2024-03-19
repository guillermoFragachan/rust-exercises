pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => self.score & 1 != 0,
            Allergen::Peanuts => self.score & 2 != 0,
            Allergen::Shellfish => self.score & 4 != 0,
            Allergen::Strawberries => self.score & 8 != 0,
            Allergen::Tomatoes => self.score & 16 != 0,
            Allergen::Chocolate => self.score & 32 != 0,
            Allergen::Pollen => self.score & 64 != 0,
            Allergen::Cats => self.score & 128 != 0,
        }
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();

        if self.is_allergic_to(&Allergen::Eggs) {
            allergies.push(Allergen::Eggs);
        }
        if self.is_allergic_to(&Allergen::Peanuts) {
            allergies.push(Allergen::Peanuts);
        }
        if self.is_allergic_to(&Allergen::Shellfish) {
            allergies.push(Allergen::Shellfish);
        }
        if self.is_allergic_to(&Allergen::Strawberries) {
            allergies.push(Allergen::Strawberries);
        }
        if self.is_allergic_to(&Allergen::Tomatoes) {
            allergies.push(Allergen::Tomatoes);
        }
        if self.is_allergic_to(&Allergen::Chocolate) {
            allergies.push(Allergen::Chocolate);
        }
        if self.is_allergic_to(&Allergen::Pollen) {
            allergies.push(Allergen::Pollen);
        }
        if self.is_allergic_to(&Allergen::Cats) {
            allergies.push(Allergen::Cats);
        }

        allergies
    }
}
