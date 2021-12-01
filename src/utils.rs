/*!
# Utilities

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::calculator::*;

pub const HALF_DIVISOR:f64 = 2.0;
pub const THIRD_DIVISOR:f64 = 3.0;
pub const QUARTER_DIVISOR:f64 = 4.0;
pub const EIGHT_DIVISOR:f64 = 8.0;
pub const TWELFTH:f64 = 12.0;
pub const ROUNDNESS:f64 = 0.7;
/*
Built in function makes the half circle down portion of a mouth
*/
pub fn make_down_part(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Path {
    let mouth = Data::new()
            .move_to((x, y))
            .cubic_curve_by((0.0, h, w, h, w, 0.0))
            .close();
    Path::new()
         .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
         .set("d", mouth)
}
/*
Make an upside-down bowl shape
*/
pub fn make_up_half(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Path {
    let mouth = Data::new()
            .move_to((x, y))
            .cubic_curve_by((0.0, -h, w, -h, w, 0.0))
            .close();
    Path::new()
        .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
         .set("d", mouth)
}
/*
Make an upside-down tear shape
*/
pub fn make_up_tear(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Path {
    let half_w:f64 = get_half(w);
    let mid:f64 = x + half_w;
    let mid_y:f64 = y + get_center(h);
    let tear = Data::new()
            .move_to((mid, y))
            //smooth_quadratic_curve_by
            .cubic_curve_by((x, mid_y, x + w, mid_y, mid, y))
            .close();
    Path::new()
        .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
         .set("d", tear)
}

/*
Make an ellipse
*/
pub fn make_ellipse(x:f64, y:f64, w:f64, h:f64, color:&str) -> Ellipse {
    let cx:f64 = get_center(w) + x;
    let cy:f64 = get_center(h) + y;
    let rx:f64 = get_center(w);
    let ry:f64 = get_center(h);
    let mut style:String = "opacity:1;fill:".to_string();
    style.push_str(color);
    Ellipse::new()
        .set("style", style.as_str())
        .set("cx", cx)
        .set("cy", cy)
        .set("rx", rx)
        .set("ry", ry)
}

/*
Make an ellipse that is semi-transparent for a shadow effect
*/
pub fn make_ellipse_opacity(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Ellipse {
    let cx:f64 = get_center(w) + x;
    let cy:f64 = get_center(h) + y;
    let rx:f64 = get_center(w);
    let ry:f64 = get_center(h);
    let mut style:String = "opacity:".to_string();
    style.push_str(opacity.to_string().as_str());
    style.push_str(";fill:");
    style.push_str(color);
    style.push(';');
    Ellipse::new()
        .set("style", style.as_str())
        .set("cx", cx)
        .set("cy", cy)
        .set("rx", rx)
        .set("ry", ry)
}
/*
Make an ellipse that is semi-transparent for a shadow effect
*/
pub fn make_ellipse_shadow(x:f64, y:f64, w:f64, h:f64, color:&str) -> Ellipse {
    make_ellipse_opacity(x,y,w,h,color,0.4)
}

/*
Make a line
*/
pub fn make_line(x:f64, y:f64, x2:f64, y2:f64, color:&str, opacity:f64) -> Path {
    let line = Data::new()
            .move_to((x, y))
            .line_to((x2, y))
            .line_to((x2, y2))
            .line_to((x, y2))
            .close();
    Path::new()
         .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
        .set("d", line)
}
/*
Make a tooth-like triangle
*/
pub fn make_slant(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64, left_facing:bool) -> Path {
    let x2:f64 = x + w;
    let y1:f64;
    let y2:f64;
    let y3:f64;
    if left_facing {
        y1 = y + h;
        y2 = y1 - h;
        y3 = y2 - h;
    } else{
        y1 = y - h;
        y2 = y1 + h;
        y3 = y2 + h;
    }
    let line = Data::new()
            .move_to((x, y1))
            .line_to((x2, y2))
            .line_to((x2, y3))
            .line_to((x, y2))
            .close();
    Path::new()
         .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
        .set("d", line)
}

/*
Make a tooth-like triangle
*/
pub fn make_sharp(x:f64, y:f64, w:f64, color:&str, opacity:f64) -> Path {
    let tooth = Data::new()
            .move_to((x, y))
            .line_to((x + w, y))
            .line_to((x + get_center(w), y + w))
            .close();
    Path::new()
         .set("fill", color)
         .set("opacity", opacity.to_string().as_str())
        .set("d", tooth)
}
/*
Make a rectangle with specific roundness
*/
pub fn make_rectangle_roundness(x:f64, y:f64, w:f64, h:f64, color:&str, roundness:f64) -> Rectangle {
    Rectangle::new()
          .set("fill", color)
          .set("x", x.to_string().as_str())
          .set("y", y.to_string().as_str())
          .set("width", w.to_string().as_str())
          .set("height", h.to_string().as_str())
          .set("rx",roundness.to_string().as_str())
          .set("opacity", "1.0")
}
/*
Make a rectangle with opacity
*/
pub fn make_rectangle_opacity(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Rectangle {
    Rectangle::new()
          .set("fill", color)
          .set("x", x.to_string().as_str())
          .set("y", y.to_string().as_str())
          .set("width", w.to_string().as_str())
          .set("height", h.to_string().as_str())
          .set("rx","0.4")
          .set("opacity", opacity.to_string().as_str())
}
/*
Make a rectangle (like teeth)
*/
pub fn make_rectangle(x:f64, y:f64, w:f64, h:f64, color:&str) -> Rectangle {
    Rectangle::new()
          .set("fill", color)
          .set("x", x.to_string().as_str())
          .set("y", y.to_string().as_str())
          .set("width", w.to_string().as_str())
          .set("height", h.to_string().as_str())
          .set("rx","0.4")
}
