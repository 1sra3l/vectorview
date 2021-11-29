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
    make_eye_shiny(x, y, w, h, color)
    //make_eye_traditional(x, y, w, h, color)
    //make_brock(x,y,w,h,true)
}
/*
Make the default animal eye
This function calls `make_eye_traditional`

*/
pub fn make_eye_animal_default(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    make_eye_feline(x, y, w, h, color)

}
/*
Make the Brock "eye"
*/
pub fn make_brock(x:f64, y:f64, w:f64, h:f64, left:bool) -> Group {
    let y = y + (h / 2.0);
    let h = h / 7.0;
    Group::new().add(make_rectangle_roundness(x, y, w, h, "black", 1.0))
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
    Group::new()
          .add(shadow)
          .add(white)
          .add(iris)
          .add(pupil)
}
/*
Make the shiny no iris eye
*/
pub fn make_eye_bw_shiny(x:f64, y:f64, w:f64, h:f64) -> Group {
    let white = make_ellipse(x,y,w,h,"white");
    let shadow = make_ellipse_shadow(x,y + 1.0,w,h,"black");
    let pupil_x:f64 = x + (w / 4.0) + (w / 10.0);
    let pupil_y:f64 = y + (h /4.0) -  (h / 5.0);
    let mut pupil_w:f64 = w / 2.0;
    let mut pupil_h:f64 = h / 2.0;
    if pupil_w <= 0.0 {
        pupil_w += 0.5;
    }
    if pupil_h <= 0.0 {
        pupil_h += 0.5;
    }
    let shine_r = pupil_w / 4.0;
    let shine_buf = pupil_h / 5.0;
    let pupil_wx = pupil_w + pupil_x - shine_r - (shine_buf * 2.0);
    let pupil_wy = pupil_h + pupil_y - shine_r - (shine_buf * 2.0);
    let pupil = make_ellipse(pupil_x, pupil_y, pupil_w, pupil_w, "black");
    let shine1 = make_ellipse(pupil_x + shine_buf, pupil_y + shine_buf, shine_r, shine_r, "white");
    let shine2 = make_ellipse(pupil_wx, pupil_wy, shine_r + shine_buf, shine_r + shine_buf, "white");
    Group::new()
          .add(shadow)
          .add(white)
          .add(pupil)
          .add(shine1)
          .add(shine2)
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
    Group::new()
          .add(shadow)
          .add(white)
          .add(iris)
          .add(pupil)
}

/*
Make the shiny no iris eye
*/
pub fn make_eye_shiny(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    make_eye_shiny_color(x,y,w,h, color, "white")
}
/*
Make the shiny no iris eye
*/
pub fn make_eye_shiny_color(x:f64, y:f64, w:f64, h:f64, color:&str, white_color:&str) -> Group {
    let white = make_ellipse(x, y, w, h, white_color);
    let shadow = make_ellipse_shadow(x,y + 1.0,w,h,"black");
    let mut iris_w:f64 = w - (w / 5.0);
    let mut iris_h:f64 = h - (h / 5.0);
    if iris_w <= 0.0 {
        iris_w += 0.5;
    }
    if iris_h <= 0.0 {
        iris_h += 0.5;
    }
    let iris_x:f64 = x + (w / 2.0) - (iris_w / 2.0);
    let iris_y:f64 = y + (h /2.0) -  (iris_h / 2.0);
    let mut pupil_w:f64 = w - (w / 2.0);
    let mut pupil_h:f64 = h - (h / 2.0);
    if pupil_w <= 0.0 {
        pupil_w += 0.2;
    }
    if pupil_h <= 0.0 {
        pupil_h += 0.2;
    }
    let pupil_x:f64 = x + (w / 2.0) - (pupil_w / 2.0);
    let pupil_y:f64 = y + (h /2.0) -  (pupil_h / 2.0);
    let shine_r = pupil_w / 4.0;
    let shine_buf = pupil_h / 5.0;
    let iris = make_ellipse(iris_x, iris_y, iris_w, iris_w, "purple");
    let pupil_wx = pupil_w + pupil_x - shine_r - (shine_buf * 2.0);
    let pupil_wy = pupil_h + pupil_y - shine_r - (shine_buf * 2.0);
    let pupil = make_ellipse(pupil_x, pupil_y, pupil_w, pupil_w, "black");
    let shine1 = make_ellipse(pupil_x + shine_buf, pupil_y + shine_buf, shine_r, shine_r, "white");
    let shine2 = make_ellipse(pupil_wx, pupil_wy, shine_r + shine_buf, shine_r + shine_buf, "white");
    Group::new()
          .add(shadow)
          .add(white)
          .add(iris)
          .add(pupil)
          .add(shine1)
          .add(shine2)
}
/*
Make the simple no iris eye
*/
pub fn make_eye_feline(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    let bg = make_ellipse(x,y,w,h,color);
    let shadow = make_ellipse_shadow(x,y + 1.0,w,h,"black");
    let pupil_x:f64 = x + (w / 4.0);
    let mut pupil_w:f64 = w / 2.0;
    let shine_r:f64 = h / 4.0;
    let shine_buf:f64 = pupil_w / 5.0;
    if pupil_w <= 0.0 {
        pupil_w += 0.5;
    }
    let pupil = make_ellipse(pupil_x, y, pupil_w, h, "black");
    let shine = make_ellipse(pupil_x + shine_buf, y + shine_buf, shine_r, shine_r, "white");
    Group::new()
          .add(shadow)
          .add(bg)
          .add(pupil)
          .add(shine)
}
