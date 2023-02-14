use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::character::stats::Stat;

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy)]
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
    Torso,
    Brain,
    LeftLung,
    RightLung,
    Heart,
    Groin,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy)]
pub enum PartType {
    Natural,
    Wood,
    Cybernetic,
    Mechanical,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Copy)]
pub enum PartStatus {
    Healthy,
    Scratched,
    Damaged,
    Broken,
    Disrupted,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct BodyPart {
    pub bodypart_type: BodyPartType,
    pub current_health_points: Stat,
    pub total_health_points: Stat,
    pub modified_health_points: Stat,
    pub part_type: PartType,
    pub part_status: PartStatus,
}

impl BodyPart {
    pub fn new(body_type: BodyPartType, total_health_points: i16, part_type: PartType, part_status: PartStatus) -> Self {
        return BodyPart {
            bodypart_type: body_type,
            current_health_points: Stat::HealthPoints(total_health_points),
            total_health_points: Stat::HealthPoints(total_health_points),
            modified_health_points: Stat::HealthPoints(total_health_points),
            part_type: PartType::Natural,
            part_status: PartStatus::Healthy,
        };
    }

    pub fn add_current_health_points(&mut self, value: i16) {
        let current_value = self.get_current_healh_points();
        let total_value: i16 = self.get_total_health_points();
        let max_value: i16 = (total_value * self.get_percent_of_part_status() as i16 / 100) as i16;

        let new_value: i16 = current_value + value;
        if new_value < max_value {
            self.current_health_points = Stat::HealthPoints(new_value);
        } else {
            self.current_health_points = Stat::HealthPoints(max_value);
        }
    }

    pub fn substruct_current_health_points(&mut self, value: i16) {
        let current_health_points = self.get_current_healh_points();
        let new_value = current_health_points - value;
        self.current_health_points = Stat::HealthPoints(new_value);
    }

    pub fn add_modified_health_points(&mut self, value: i16) {
        let modified_health_points = self.get_modified_health_points();
        let new_value = modified_health_points + value;
        self.total_health_points = Stat::HealthPoints(new_value);
        self.modified_health_points = Stat::HealthPoints(new_value);
        //self.add_current_health_points(value);
    }

    pub fn substruct_modified_health_points(&mut self, value: i16) {
        // we can change modidified value to biggest "-", but we can't disrupt part type, min total hp and curent hp will be equal 1;
        let new_value = self.get_modified_health_points() - value;
        self.modified_health_points = Stat::HealthPoints(new_value);
        let total_health = self.get_total_health_points();
        let new_total_health = total_health - value;
        let current_health_points: i16 = self.get_current_healh_points();


        if new_total_health <= 0 {
            self.total_health_points = Stat::HealthPoints(1);
            self.current_health_points = Stat::HealthPoints(1);
        }else if new_total_health < current_health_points {

        }else{
            self.health_points.1 = new_total_health;
            let current_health = self.health_points.0;
            let new_current_health = current_health - value;
            if new_current_health <= 0 {
                self.health_points.0 = 1;
            }else{
                self.health_points.0 = new_current_health;
            }
        }
    }

    fn get_current_healh_points(&self)  -> i16{
        match self.current_health_points {
            Stat::HealthPoints(v) => v,
            _ => {
                println!( " Func get_current_health_points form body_part.rs; {:?} is return 0, because wrong enum assign", self.bodypart_type );
                0
            }                
        }
    }

    fn get_total_health_points(&self) -> i16{
        match self.total_health_points {
            Stat::HealthPoints(v) => v,
            _ => {
                println!( " Func get_total_health_points form body_part.rs; {:?} is return 0, because wrong enum assign", self.bodypart_type );
                0
            } 
        }
    }

    fn get_modified_health_points(&self) -> i16{
        match self.modified_health_points {
            Stat::HealthPoints(v) => v,
            _ => {
                println!( " Func get_modified_health_points form body_part.rs; {:?} is return 0, because wrong enum assign", self.bodypart_type );
                0
            } 
        }
    }

    fn get_part_status_from_percent(&self, percent: i8) -> PartStatus {
        match self.part_type {
            PartType::Natural => {
                let result = if percent > 90 {
                    PartStatus::Healthy
                } else if percent <= 90 && percent > 60 {
                    PartStatus::Scratched
                } else if percent <= 60 && percent > 30 {
                    PartStatus::Damaged
                } else if percent <= 30 && percent > 0 {
                    PartStatus::Broken
                } else {
                    PartStatus::Disrupted
                };
                return result;
            }
            PartType::Wood => {
                let result = if percent > 90 {
                    PartStatus::Healthy
                } else if percent <= 90 && percent > 80 {
                    PartStatus::Scratched
                } else if percent <= 80 && percent > 50 {
                    PartStatus::Damaged
                } else if percent <= 50 && percent > 0 {
                    PartStatus::Broken
                } else {
                    PartStatus::Disrupted
                };
                return result;
            }
            PartType::Mechanical => {
                let result = if percent > 90 {
                    PartStatus::Healthy
                } else if percent <= 90 && percent > 50 {
                    PartStatus::Scratched
                } else if percent <= 50 && percent > 20 {
                    PartStatus::Damaged
                } else if percent <= 20 && percent > 0 {
                    PartStatus::Broken
                } else {
                    PartStatus::Disrupted
                };
                return result;
            }
            PartType::Cybernetic => {
                let result = if percent > 90 {
                    PartStatus::Healthy
                } else if percent <= 90 && percent > 30 {
                    PartStatus::Scratched
                } else if percent <= 30 && percent > 10 {
                    PartStatus::Damaged
                } else if percent <= 10 && percent > 0 {
                    PartStatus::Broken
                } else {
                    PartStatus::Disrupted
                };
                return result;
            }
        }
    }

    fn get_percent_of_part_status(&self) -> i8 {
        let part_status = &self.part_status;
        match self.part_type {
            PartType::Natural => {
                let result = match part_status {
                    PartStatus::Disrupted => 0,
                    PartStatus::Broken => 30,
                    PartStatus::Damaged => 60,
                    PartStatus::Scratched => 90,
                    PartStatus::Healthy => 100,
                };
                return result;
            }
            PartType::Wood => {
                let result = match part_status {
                    PartStatus::Disrupted => 0,
                    PartStatus::Broken => 50,
                    PartStatus::Damaged => 80,
                    PartStatus::Scratched => 90,
                    PartStatus::Healthy => 100,
                };
                return result;
            }
            PartType::Mechanical => {
                let result = match part_status {
                    PartStatus::Disrupted => 0,
                    PartStatus::Broken => 20,
                    PartStatus::Damaged => 50,
                    PartStatus::Scratched => 90,
                    PartStatus::Healthy => 100,
                };
                return result;
            }
            PartType::Cybernetic => {
                let result = match part_status {
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
}
