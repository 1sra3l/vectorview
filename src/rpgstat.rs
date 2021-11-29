/*!
# RPG stat library integration

*/
use svg::node::element::Group as SvgGroup;

use crate::svg::{VectorView, *};
use crate::enums::{Teeth, Expression};

#[cfg(feature = "rpg")]
use rpgstat::effect::Normal as NormalEffect;
/*
effect::Normal icons
*/
#[cfg(feature = "rpg")]
impl VectorView for NormalEffect{
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> SvgGroup {
        let icon = SvgGroup::new()
                            .add(self.make_rectangle(x, y, w, h, ""));
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
