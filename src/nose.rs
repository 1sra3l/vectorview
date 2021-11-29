/*!
# Nose

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::utils::*;

/*
Make the default animal nose
*/
pub fn make_animal_nose_default(x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
    make_animal_nose_traditional(x, y, w, h, nose_color)
}
/*
Make the default human nose
*/
pub fn make_nose_default(x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
    make_nose_traditional(x, y, w, h, nose_color)
}

/*
Make the traditional animal nose
*/
pub fn make_animal_nose_traditional(x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
    let h = h / 2.0;
    let y = y + h;
    let nose = make_ellipse(x, y, w, h, nose_color);
    let spacer = w / 6.0;
    let shine_w = w - spacer - spacer;
    let shine_h = h / 3.0;
    let y = y + (shine_h / 5.0);
    let nose_shine = make_ellipse_opacity(x + spacer, y, shine_w, shine_h, "white",0.2);
    Group::new().add(nose).add(nose_shine)
}
/*
Make nose
*/
pub fn make_nose_traditional(x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
    let nose_w = w / 3.0;
    let nose_x = x + nose_w;
    let nose = make_rectangle_roundness(nose_x, y, nose_w, h, nose_color, 5.0);
    let nos_h:f64 = h / 3.0;
    let nos_y:f64 = y + h - nos_h;
    let nostrils = make_ellipse(x, nos_y, w, nos_h, nose_color);
    Group::new().add(nose).add(nostrils)
}
/*
Make a muppet-like nose
*/
pub fn make_nose_muppet(x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
    let nose = make_rectangle_roundness(x, y, w, h, nose_color, 5.0);
    Group::new().add(nose)
}
