use crate::genetics::GeneticPossibilities;

pub const PLANT_GROW_TIME_IN_SECONDS: i16 = 3600;
pub const CHOICES: [GeneticPossibilities; 4] = [
    GeneticPossibilities::Empty,
    GeneticPossibilities::Growth,
    GeneticPossibilities::Yield,
    GeneticPossibilities::Water,
];
pub const DEFAULT_WEIGHTS: [f32; 4] = [0.40, 0.20, 0.20, 0.20];
