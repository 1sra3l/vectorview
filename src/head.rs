/*!
# Head Shapes

The module controls the drawing of the shapes of a person's head
*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::utils::*;

/*
Make default Head shape
*/
pub fn make_head_default(x:f64, y:f64, w:f64, h:f64, skin_color:&str) -> Group {
    make_head_traditional(x, y, w, h, skin_color)
}
/*
Make traditional Head shape
*/
pub fn make_head_traditional(x:f64, y:f64, w:f64, h:f64, skin_color:&str) -> Group {    
    let center_x:f64 = x + (w / 2.0);
    let chin:f64 = h / 4.0;
    let face_h:f64 = h - chin;
    let jaw_h:f64 = h - face_h;
    let jaw_w:f64 = (w / 7.0) * 3.0;
    let jaw_y:f64 = y + h - chin;
    let cheek_w:f64 = (w / 5.0) * 4.0;
    let jaw_x:f64 = x + ((w - jaw_w) / 2.0);
    let face = make_ellipse(x, y, w, face_h, skin_color);
    let cheeks = make_ellipse(center_x - (cheek_w / 2.0), y, cheek_w, h, skin_color);
    let jaw = make_rectangle_roundness(jaw_x, jaw_y, jaw_w, jaw_h, skin_color, 7.0);
    Group::new()
          .add(cheeks)
          .add(face)
          .add(jaw)
}
