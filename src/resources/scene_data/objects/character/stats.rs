use serde::{Serialize, Deserialize};

const MIN_STAT_VALUE: i16 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, Copy )]
pub enum Stat{
    Strength(i16),
    Endurance(i16),
    Intellect(i16),
    Agility(i16),
    //Vitality(i16),
    //Mobility(i16),
    HealthPoints(i16),
}

impl Stat {
    pub fn get_stat(&self) -> i16 {
        match *self{
            Stat::Strength(v)
            | Stat::Endurance(v)
            | Stat::Intellect(v)
            | Stat::Agility(v)
            | Stat::HealthPoints(v) => v
        }
    }

    pub fn set_stat(&mut self, value: i16) {
        match *self {
            Stat::Strength(mut v)
            | Stat::Endurance(mut v)
            | Stat::Intellect(mut v)
            | Stat::Agility(mut v)
            | Stat::HealthPoints(mut v) => v = value
        }
    }
}

impl Default for Stat{
    fn default() -> Self{
        Stat::HealthPoints(0)
    }
}