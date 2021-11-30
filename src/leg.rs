/*!
# Leg


*/
use svg::node::element::Group;
use crate::utils::*;
use crate::enums::Pose;
use crate::calculator::*;
/*
#  maker


*/
pub fn make_leg_default(x:f64, y:f64, w:f64, h:f64, color:&str, left:bool) -> Group {
    let y = get_hip_y(y, h);
    let w = get_arm_width(w);
    let h = get_arm_length(h);
    let x = get_arm_x(x, w, left);
    let pose = Pose::None;
    Group::new()
}
/*
#  maker


*/
pub fn make_(x:f64, y:f64, w:f64, h:f64, color:&str, pose:Pose, left:bool) -> Group {
    Group::new()
          .add(make_ellipse(x, y, w, h, color))
}
