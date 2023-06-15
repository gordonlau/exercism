pub struct Allergies{
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl Allergen{


    fn all_allergens() -> Vec<Allergen>{
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ]
    }
}


impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_num = *allergen as u32;
        (self.score & allergen_num)  > 0 
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for allergen in Allergen::all_allergens(){
            if self.is_allergic_to(&allergen){
                allergies.push(allergen)
            }
        }
        allergies
    }
}
