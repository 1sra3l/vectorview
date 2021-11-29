/*!
# Utilities

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;

pub const HALF_DIVISOR:f64 = 2.0;
pub const THIRD_DIVISOR:f64 = 3.0;
pub const QUARTER_DIVISOR:f64 = 4.0;
pub const EIGHT_DIVISOR:f64 = 8.0;
pub const TWELFTH:f64 = 12.0;

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
Make an ellipse
*/
pub fn make_ellipse(x:f64, y:f64, w:f64, h:f64, color:&str) -> Ellipse {
    let cx:f64 = (w / 2.0) + x;
    let cy:f64 = (h / 2.0) + y;
    let rx:f64 = w / 2.0;
    let ry:f64 = h / 2.0;
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
    let cx:f64 = (w / 2.0) + x;
    let cy:f64 = (h / 2.0) + y;
    let rx:f64 = w / 2.0;
    let ry:f64 = h / 2.0;
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
            .line_to((x + (w / 2.0), y + w))
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
