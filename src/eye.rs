/*!
# Eye

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::utils::*;
/*
Make the default eye
This function calls `make_eye_traditional`

*/
pub fn make_eye_default(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    make_eye_traditional(x, y, w, h, color)
}

/*
Make the simple no iris eye
*/
pub fn make_eye_simple(x:f64, y:f64, w:f64, h:f64) -> Group {
    let white = make_ellipse(x,y,w,h,"white");
    let shadow = make_ellipse_shadow(x,y + 1.0,w,h,"black");
    let iris_x:f64 = x + (w / 4.0) + (w / 10.0);
    let iris_y:f64 = y + (h /4.0) -  (h / 5.0);
    let mut iris_w:f64 = w / 2.0;
    let mut iris_h:f64 = h / 2.0;
    if iris_w <= 0.0 {
        iris_w += 0.5;
}
    if iris_h <= 0.0 {
        iris_h += 0.5;
}
    let iris = make_ellipse(iris_x, iris_y, iris_w, iris_w, "black");
    let pupil = make_ellipse(iris_x + (iris_w / 4.0), iris_y + (iris_h / 4.0), w / 4.0, h / 4.0, "white");
    let g = Group::new()
            .add(shadow)
            .add(white)
            .add(iris)
            .add(pupil);
    g
}


/*
Make the traditional eye looking straight on with an iris
*/
pub fn make_eye_traditional(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    let white = make_ellipse(x,y,w,h,"white");
    let shadow = make_ellipse_shadow(x,y + 1.0,w,h,"black");
    let iris_x:f64 = x + (w / 4.0) + (w / 10.0);
    let iris_y:f64 = y + (h /4.0) -  (h / 5.0);
    let mut iris_w:f64 = w / 2.0;
    let mut iris_h:f64 = h / 2.0;
    if iris_w <= 0.0 {
        iris_w += 0.5;
}
    if iris_h <= 0.0 {
        iris_h += 0.5;
}
    let iris = make_ellipse(iris_x, iris_y, iris_w, iris_w, color);
    let pupil = make_ellipse(iris_x + (iris_w / 4.0), iris_y + (iris_h / 4.0), w / 4.0, h / 4.0, "black");
    let g = Group::new()
            .add(shadow)
            .add(white)
            .add(iris)
            .add(pupil);
    g
}
