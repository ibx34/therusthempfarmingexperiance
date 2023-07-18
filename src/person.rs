#[derive(Debug)]
pub struct PersonStats {
    /// Every tick of this stat is an extra 0.5% off of the growth time of a plant.
    /// Say you frigging LOVE plants and have a plant_flatteringness stat of 20 and you have a plant
    /// with no G traits (so the plant takes an hour to grow) then it will take 37.5 minutes instead of the full
    /// 60 minutes. The max stat that a person can have is 32.
    ///
    /// The math? I think this is correct
    /// 20 / 35 = 0.625
    /// 0.625 of 3600 is 22.5
    /// 22.5 * 100 = 2250
    /// 2250 is the amount of time, in seconds, it takes for a plant to grow.
    ///
    /// THIS STATE DOES NOT EFFECT PICKING A PLANT SEED!!!!!!
    pub plant_flatteringness: f32,
    /// Every tick of this will take 0.5% off of the chance of getting an empty genetic slot.
    pub person_handsoftness: f32,
}

#[derive(Debug)]
pub struct Person {
    pub id: i64,
    pub stats: PersonStats,
}
