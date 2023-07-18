use crate::{
    genetics::{GeneticPossibilities, Genetics},
    person::Person,
};

/// The genetics of seeds are generated when they are PICKED not CREATED.
#[derive(Debug)]
pub struct Seed {
    pub genetics: Genetics,
    /// This is NOT a racist thing. :rolling_eyes:
    pub picked_by: i64,
}

impl Seed {
    pub fn pick(p: Person) -> Self {
        let genetics = (0..6)
            .map(|_| GeneticPossibilities::generate_random(&p))
            .collect::<Vec<GeneticPossibilities>>();
        let genetics = genetics.as_slice();

        Seed {
            picked_by: p.id,
            genetics: Genetics(
                genetics[0].to_owned(),
                genetics[1].to_owned(),
                genetics[2].to_owned(),
                genetics[3].to_owned(),
                genetics[4].to_owned(),
                genetics[5].to_owned(),
            ),
        }
    }
}
