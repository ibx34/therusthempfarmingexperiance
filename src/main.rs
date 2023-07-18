pub mod consts;
pub mod genetics;
pub mod person;
pub mod seed;

use crate::{
    person::{Person, PersonStats},
    seed::Seed,
};

fn main() {
    let person = Person {
        id: 0,
        stats: PersonStats {
            plant_flatteringness: 0.0,
            person_handsoftness: 0.0,
        },
    };
    let seed = Seed::pick(person);
    println!("{:?}", seed);
}
