use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy, Hash, Default)]
pub enum BodyPartType {
    Head,
    LeftEye,
    RightEye,
    Mouth,
    Nose,
    LeftArm,
    LeftWrist,
    RightArm,
    RightWrist,
    LeftFoot,
    LeftSole,
    RightFoot,
    RightSole,
    #[default]
    Torso,
    Brain,
    LeftLung,
    RightLung,
    Heart,
    Groin,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy, Default, Hash)]
pub enum PartType {
    #[default]
    Natural,
    Wood,
    Cybernetic,
    Mechanical,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy, Default)]
pub enum PartStatus {
    #[default]
    Healthy,
    Scratched,
    Damaged,
    Broken,
    Disrupted,
}
#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Default, Hash)]
pub enum HealthPoints{
    #[default]
    Current,
    Modified,
    Total,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BodyPart {
    pub bodypart_type: BodyPartType,
    pub health_points: HashMap<HealthPoints, i16>,
    pub part_type: PartType,
    pub part_status: PartStatus,
}

impl BodyPart {
    pub fn get_current_health_points(&self) -> i16 {
        match self.health_points.get(&HealthPoints::Current) {
            Some(v) => return *v,
            None => { 
                println!("Can't get current health points from '{:?}', return 0", self.bodypart_type); 
                return 0;
            }
        };
    }

    pub fn get_total_health_points(&self) -> i16 {
        match self.health_points.get(&HealthPoints::Total) {
            Some(v) => return *v,
            None => { 
                println!("Can't get total health points from '{:?}', return 0", self.bodypart_type); 
                return 0;
            }
        };
    }

    pub fn get_modified_health_points(&self) -> i16 {
        match self.health_points.get(&HealthPoints::Modified) {
            Some(v) => return *v,
            None => { 
                println!("Can't get modified health points from '{:?}', return 0", self.bodypart_type); 
                return 0;
            }
        };
    }

    pub fn set_current_health_points(&mut self, value: i16) {
        // create new key if not exist, or modify current key value, if existed;
        self.health_points.entry(HealthPoints::Current).and_modify(|key: &mut i16| *key = value).or_insert(value);
    }

    pub fn set_total_health_points(&mut self, value: i16) {
        // create new key if not exist, or modify current key value, if existed;
        self.health_points.entry(HealthPoints::Total).and_modify(|key: &mut i16| *key = value).or_insert(value);
    }

    pub fn set_modified_health_points(&mut self, value: i16) {
        // create new key if not exist, or modify current key value, if existed;
        self.health_points.entry(HealthPoints::Modified).and_modify(|key: &mut i16| *key = value).or_insert(value);
    }
}

pub fn change_current_health_points(bodypart: &mut BodyPart, value: i16) {
    let current_value = bodypart.get_current_health_points();
    let total_value: i16 = bodypart.get_total_health_points();
    let max_value: i16 =
        (total_value * get_percent_of_part_status(bodypart, Option::None) as i16 / 100) as i16;

    let new_value: i16 = current_value + value;
    if new_value < max_value {
        bodypart.set_current_health_points(new_value);
    } else {
        bodypart.set_current_health_points(max_value);
    }
}

pub fn change_modified_health_points(bodypart: &mut BodyPart, value: i16) {
    // we can change modidified value to biggest "-", but we can't disrupt part type, min total hp and curent hp will be equal 1;
    let new_value = bodypart.get_modified_health_points() + value;
    bodypart.set_modified_health_points(new_value);
    let total_health = bodypart.get_total_health_points();
    let new_total_health = total_health + value;
    let current_health_points: i16 = bodypart.get_current_health_points();
    let new_current_health_points: i16 = bodypart.get_current_health_points() + value;

    // calc total health points
    if new_total_health <= 0 && bodypart.part_status != PartStatus::Disrupted {
        bodypart.set_total_health_points(1);
        bodypart.set_current_health_points(1);
    } else if new_total_health <= current_health_points
        && bodypart.part_status != PartStatus::Disrupted
    {
        bodypart.set_total_health_points(new_total_health);
    } else {
        bodypart.set_total_health_points(new_total_health);
    }

    //calc current health pints
    let percent = get_percent_of_part_status(bodypart, Option::None);
    let part_status = get_part_status_from_percent(bodypart, Option::Some(percent));
    let lower_percent = match part_status {
        PartStatus::Healthy => {
            get_percent_of_part_status(bodypart, Option::Some(&PartStatus::Scratched))
        }
        PartStatus::Scratched => {
            get_percent_of_part_status(bodypart, Option::Some(&PartStatus::Damaged))
        }
        PartStatus::Damaged => {
            get_percent_of_part_status(bodypart, Option::Some(&PartStatus::Broken))
        }
        PartStatus::Broken => 0,
        PartStatus::Disrupted => return,
    };

    let new_min_current_health_points = new_total_health * lower_percent as i16 / 100;
    let new_max_current_health_points = new_total_health * percent as i16 / 100;

    if new_current_health_points >= new_max_current_health_points {
        // when part don't get damage at all;
        bodypart.set_current_health_points(new_current_health_points);
    } else if new_current_health_points < new_max_current_health_points
        && new_current_health_points >= new_min_current_health_points
    {
        bodypart.set_current_health_points(new_current_health_points);
    } else {
        bodypart.set_current_health_points(new_min_current_health_points);
    }
}

pub fn set_health_points(bodypart: &mut BodyPart, value: i16) {
    bodypart.set_modified_health_points(value);
    bodypart.set_total_health_points(value);
    bodypart.set_current_health_points(value);
}

fn get_part_status_from_percent(bodypart: &BodyPart, percent: Option<i8>) -> PartStatus {
    let new_percent = match percent {
        Option::Some(v) => v,
        Option::None => {
            let current_health = bodypart.get_current_health_points();
            let total_health = bodypart.get_total_health_points();
            (current_health * 100 / total_health) as i8
        }
    };

    match bodypart.part_type {
        PartType::Natural => {
            let result = if new_percent > 90 {
                PartStatus::Healthy
            } else if new_percent <= 90 && new_percent > 60 {
                PartStatus::Scratched
            } else if new_percent <= 60 && new_percent > 30 {
                PartStatus::Damaged
            } else if new_percent <= 30 && new_percent > 0 {
                PartStatus::Broken
            } else {
                PartStatus::Disrupted
            };
            return result;
        }
        PartType::Wood => {
            let result = if new_percent > 90 {
                PartStatus::Healthy
            } else if new_percent <= 90 && new_percent > 80 {
                PartStatus::Scratched
            } else if new_percent <= 80 && new_percent > 50 {
                PartStatus::Damaged
            } else if new_percent <= 50 && new_percent > 0 {
                PartStatus::Broken
            } else {
                PartStatus::Disrupted
            };
            return result;
        }
        PartType::Mechanical => {
            let result = if new_percent > 90 {
                PartStatus::Healthy
            } else if new_percent <= 90 && new_percent > 50 {
                PartStatus::Scratched
            } else if new_percent <= 50 && new_percent > 20 {
                PartStatus::Damaged
            } else if new_percent <= 20 && new_percent > 0 {
                PartStatus::Broken
            } else {
                PartStatus::Disrupted
            };
            return result;
        }
        PartType::Cybernetic => {
            let result = if new_percent > 90 {
                PartStatus::Healthy
            } else if new_percent <= 90 && new_percent > 30 {
                PartStatus::Scratched
            } else if new_percent <= 30 && new_percent > 10 {
                PartStatus::Damaged
            } else if new_percent <= 10 && new_percent > 0 {
                PartStatus::Broken
            } else {
                PartStatus::Disrupted
            };
            return result;
        }
    }
}

fn get_percent_of_part_status(bodypart: &BodyPart, part_status: Option<&PartStatus>) -> i8 {
    let new_part_status = match part_status {
        Option::Some(v) => v,
        Option::None => &bodypart.part_status,
    };

    match bodypart.part_type {
        PartType::Natural => {
            let result = match new_part_status {
                PartStatus::Disrupted => 0,
                PartStatus::Broken => 30,
                PartStatus::Damaged => 60,
                PartStatus::Scratched => 90,
                PartStatus::Healthy => 100,
            };
            return result;
        }
        PartType::Wood => {
            let result = match new_part_status {
                PartStatus::Disrupted => 0,
                PartStatus::Broken => 50,
                PartStatus::Damaged => 80,
                PartStatus::Scratched => 90,
                PartStatus::Healthy => 100,
            };
            return result;
        }
        PartType::Mechanical => {
            let result = match new_part_status {
                PartStatus::Disrupted => 0,
                PartStatus::Broken => 20,
                PartStatus::Damaged => 50,
                PartStatus::Scratched => 90,
                PartStatus::Healthy => 100,
            };
            return result;
        }
        PartType::Cybernetic => {
            let result = match new_part_status {
                PartStatus::Disrupted => 0,
                PartStatus::Broken => 10,
                PartStatus::Damaged => 30,
                PartStatus::Scratched => 90,
                PartStatus::Healthy => 100,
            };
            return result;
        }
    }
}
