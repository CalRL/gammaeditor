use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use crate::pkmn::ball::PokeBall;

pub struct CaughtBall<'a> {
    property: &'a Property
}
impl<'a> CaughtBall<'a> {
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        Some(
            Self {
                property: gvas_file.properties.get("PartyCaughtBall")?
            }
        )
    }

    pub fn get_caught_ball_at(&self, index: usize) -> Option<PokeBall> {
        let arr: &ArrayProperty = self.property.get_array()?;
        let ball_enum: String = get_caught_ball_at(arr, index)?;

        PokeBall::from_enum(ball_enum.as_str())
    }
}

pub struct CaughtBallMut<'a> {
    property: &'a mut Property
}
impl<'a> CaughtBallMut<'a> {
    pub fn new_party(gvas_file: &'a mut GvasFile) -> Option<Self> {
        Some(
            Self {
                property: gvas_file.properties.get_mut("PartyCaughtBall")?
            }
        )
    }

    pub fn set_ball_at(&mut self, poke_ball: PokeBall, index: usize) -> Result<(), String> {

        if let Some(arr) = self.property.get_array_mut() {
            if let Some(ball_enum) = get_caught_ball_at_mut(arr, index) {
                *ball_enum = poke_ball.as_enum().to_string()
            }
        }

        Ok(())
    }
}

fn get_caught_ball_at(array: &ArrayProperty, index: usize) -> Option<String> {
    let property = match array {
        ArrayProperty::Properties { properties, .. } => {
            properties.get(index)?
        }
        _ => return None
    };

    match property {
        Property::ObjectProperty(object) => {
            Some(object.value.clone())
        }
        _ => None
    }
}

fn get_caught_ball_at_mut(array: &mut ArrayProperty, index: usize) -> Option<&mut String> {
    let mut property = match array {
        ArrayProperty::Properties { properties, .. } => {
            properties.get_mut(index)?
        }
        _ => return None
    };

    match property {
        Property::ObjectProperty(object) => {
            Some(&mut object.value)
        }
        _ => None
    }
}