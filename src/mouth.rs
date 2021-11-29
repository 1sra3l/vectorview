/*!
*/
use svg::node::element::{Ellipse, Filter, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::enums::Teeth;
use crate::utils::*;
use crate::teeth::*;

/*
Make the default mouth
*/
pub fn make_mouth_default(x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
    make_smile_mouth(x, y, w, h, color, teeth)
}
/*
Make a smiling mouth
*/
pub fn make_smile_mouth(x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
    let path = make_down_part(x, y, w, h, "black", 1.0);
    let spacer = w / 24.0;
    let tooth_w:f64 = w - (2.0 * spacer);
    let tooth_h:f64 = h / 4.0;
    let tooth = make_teeth_default(x + spacer, y, tooth_w, tooth_h, color, teeth);
    Group::new()
         .add(path)
         .add(tooth)
}
