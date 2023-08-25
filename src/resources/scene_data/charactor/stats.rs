use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Stat{
    #[default]
    Strength,
    Dexterity,
    Mobility,
    Wisdom,
    Intellect,
    Luck,
    Vitality,
    Endurance,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum ExtraStat{
    #[default]
    HealthPoints,
    StaminaPoints,
}

//formulas
pub fn get_values_of_extra_stats_from_stat(stat: &Stat, value: i16) -> HashMap<ExtraStat, i16> {
    let mut result: HashMap<ExtraStat, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {},
        Stat::Dexterity => {},
        Stat::Mobility => {},
        Stat::Wisdom => {},
        Stat::Intellect => {},
        Stat::Luck => {},
        Stat::Vitality => {
            //SP VIT * 2;
            let result_value = value * 2;
            result.insert(ExtraStat::StaminaPoints, result_value);
        },
        Stat::Endurance => {
            //HP END * 2;
            let result_value = value * 2;
            result.insert(ExtraStat::HealthPoints, result_value);
        },
    }
    return result;
}
