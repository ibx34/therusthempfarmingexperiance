use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{
    consts::{CHOICES, DEFAULT_WEIGHTS},
    person::Person,
};

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum GeneticPossibilities {
    /// An empty and useless slot
    Empty = 0,
    /// The plant will take less time to grow
    Growth = 1,
    /// The plant will give more yield
    Yield = 2,
    /// Makes the plant require more water
    Water = 3,
}

impl GeneticPossibilities {
    pub fn generate_random(person: &Person) -> Self {
        let mut weights: [f32; 4] = DEFAULT_WEIGHTS.clone();
        if person.stats.person_handsoftness > 0.0 {
            let amount_to_take_off = 0.05 * person.stats.person_handsoftness;
            weights[0] = weights[0] - amount_to_take_off;
        }
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();

        CHOICES
            .get(dist.sample(&mut rng))
            .unwrap_or(&GeneticPossibilities::Empty)
            .to_owned()
    }
}

/// Genetics have 6 slots, similar to rust, in fact, i stole this from rust.
/// they can be one of: E, G, Y, W. Two of these are negative traits but I forgot them.
///
/// The number in this list is the numerical value for each trait, just fyi!
///
/// 0: E or Empty is a useless trait and is just there to fuck with you!
/// 1: G or Growth is the speed at which is a plant will grow. However, stacking too many can be a bad thing.
/// 2: Y or Yield is how much yield you get from harvesting this plant
/// 3: W or Water is how much water the plant wants. The more W traits the shittier your plant is and you should reconsider playing this shitty game
///
//         Slot     1       2       3       4       5       6
#[derive(Debug)]
pub struct Genetics(
    pub GeneticPossibilities,
    pub GeneticPossibilities,
    pub GeneticPossibilities,
    pub GeneticPossibilities,
    pub GeneticPossibilities,
    pub GeneticPossibilities,
);

impl Into<Vec<GeneticPossibilities>> for Genetics {
    fn into(self) -> Vec<GeneticPossibilities> {
        vec![self.0, self.1, self.2, self.3, self.4, self.5]
    }
}
