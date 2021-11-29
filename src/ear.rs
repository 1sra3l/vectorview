/*!
# Ear

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::utils::*;

/*
Make ear
*/
pub fn make_ear_default(x:f64, y:f64, w:f64, h:f64, ear_color:&str, left:bool) -> Group {
    let ear = make_ellipse(x, y, w, h, ear_color);
    let spacer:f64 = w / 4.0;
    let x1:f64 = x + spacer;
    let y1:f64 = y + spacer;
    let w1:f64 = w - (2.0 * spacer);
    let h1:f64 = h - (2.0 * spacer);
    let shadow = make_ellipse_shadow(x1,y1,w1,h1,"black");
    let x2:f64;
    let y2:f64 = y + (h / 2.0) - spacer;
    let small_w:f64 = w / 3.0;
    let small_h:f64 = h / 3.0;
    let half_spacer:f64 = spacer / 2.0;
    if left {
        x2 = x + w - small_w - spacer + half_spacer;
    } else {
        x2 = x1 - half_spacer;
    }
    let ear_thing = make_ellipse(x2, y2, small_w, small_h, ear_color);
    Group::new()
         .add(ear)
         .add(shadow)
         .add(ear_thing)
    }
