use crate::resources::gamedata::character::body_part::BodyPart;

pub struct HealthStats{
    pub head: Option<BodyPart>,
    pub nose: Option<BodyPart>,
    pub left_eye: Option<BodyPart>,
    pub right_eye: Option<BodyPart>,
    pub mouth: Option<BodyPart>,
    pub torso: BodyPart,
    pub left_lung: Option<BodyPart>,
    pub right_lung: Option<BodyPart>,
    pub heart: Option<BodyPart>,
    pub current_health_points: i16,
    pub total_health_points: i16,
}


