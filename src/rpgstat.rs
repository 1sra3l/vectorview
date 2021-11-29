/*!
# RPG stat library integration

*/
use svg::node::element::Group as SvgGroup;

use crate::utils::*;
use crate::icon::*;
use crate::body::{VectorView, *};
use crate::enums::{Teeth, Expression};

#[cfg(feature = "rpg")]
use rpgstat::effect::Normal as NormalEffect;
#[cfg(feature = "rpg")]
use rpgstat::types::Advanced as AdvancedType;

/*
effect::Normal icons
*/
#[cfg(feature = "rpg")]
impl VectorView for NormalEffect{
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> SvgGroup {
        let icon = SvgGroup::new()
                            .add(make_rectangle(x, y, w, h, color));
        match *self {
            NormalEffect::Poison => icon.clone()
                                        .add(self
                                        .make_skull(x,y,w,h,"green")),
            NormalEffect::Freeze => icon.clone()
                                        .add(self
                                        .make_skull(x,y,w,h,"#affbff")),
            NormalEffect::Sick => icon.clone()
                                        .add(self
                                        .make_face(x, y, w, h, "#f8ff75", "#502f07", "#afb453", "black", "white", false, Teeth::Sharp)),
            NormalEffect::Sap => icon.clone()
                                        .add(self
                                        .make_skull(x,y,w,h,"#5500ff")),
            _=>  icon.clone()
                     .add(self
                     .make_face(x, y, w, h, color.clone(), "#502f07", "orange", "black", "white", false, Teeth::Straight)),
        }
    }
    //fn make_mouth(&self, x:i32, y:i32, w:i32, h:i32, color:&str) -> SvgGroup {}
}
#[cfg(feature = "rpg")]
impl VectorView for AdvancedType {
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> SvgGroup {
        match *self {
            /*AdvancedType::Feline =>,
            AdvancedType::Canine =>,
            AdvancedType::Rodent =>,
            AdvancedType::Primate,
            AdvancedType::Bug,
            AdvancedType::Amphibian,
            AdvancedType::Reptile,
            AdvancedType::Fish,
            AdvancedType::Dragon,
            AdvancedType::Legendary,
            AdvancedType::Plasma,
            AdvancedType::Magma,
            AdvancedType::Crystal,
            AdvancedType::Laser,
            AdvancedType::Tech,
            AdvancedType::Leaf,
            AdvancedType::Patch,
            AdvancedType::Undead,
            AdvancedType::Star,
            AdvancedType::Galactic,
            AdvancedType::Kaiju,
            AdvancedType::Xeno,
            AdvancedType::Paper,
            AdvancedType::Shifter,
            AdvancedType::Gravity,
            AdvancedType::Life,
            AdvancedType::Food,
            AdvancedType::Death,
            AdvancedType::Mana,
            AdvancedType::Bubble,
            AdvancedType::Seed,
            AdvancedType::Bean,
            AdvancedType::Clay,
            AdvancedType::Steel,
            AdvancedType::Iron,
            AdvancedType::Vine,
            AdvancedType::Tree,
            AdvancedType::River,
            AdvancedType::Ocean,
            AdvancedType::Ember,
            AdvancedType::Lava,
            AdvancedType::Spark,
            AdvancedType::Lightning,
            AdvancedType::Holy,
            AdvancedType::Unholy,
            AdvancedType::Sunrise,
            AdvancedType::Sunset,
            AdvancedType::Moonrise,
            AdvancedType::Moonset,
            AdvancedType::Tornado,
            AdvancedType::Breeze,
            AdvancedType::Blustry,*/
            _=> return self.make_animal_face(x,y,w,h, color, "#fff237", "#b4584c", "black", "white", true, Teeth::Sharp),
        }
    }
}
