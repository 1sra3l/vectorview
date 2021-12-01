/*!
# Arm


*/
use svg::node::element::Group;
use crate::utils::*;
use crate::enums::Pose;
use crate::calculator::*;

/*
Make an arm
*/
pub fn make_arm_default(x:f64, y:f64, w:f64, h:f64, color:&str, left:bool) -> Group {
    let y = get_trunk_y(y, h);
//    let w = get_arm_width(w);
  //  let h = get_arm_length(h);
    
    let pose = Pose::None;
    let bicep = make_bicep(x, y , w, h, color, pose, left);
    let brach = make_brach(x, y, w, h, color, pose, left);
    Group::new()
         .add(bicep)
         .add(brach)
}

/*
# Biceps maker


*/
pub fn make_bicep(x:f64, y:f64, w:f64, h:f64, color:&str, pose:Pose, left:bool) -> Group {
    let spacer:f64 = h / TWELFTH;
    let bicep_h:f64 = get_arm_length(h);
    let bicep_w:f64 = get_arm_width(w);
    let spacer:f64 = get_deltoid_spacer(w);
    let bicep_x:f64;

    if left {
        bicep_x = x + spacer;
    } else {
        bicep_x = x + w - bicep_h - spacer;
    }
    Group::new()
          .add(make_ellipse(bicep_x, y + spacer, bicep_h, bicep_w, color))
}

/*
# Brachiordialis and pronator teres maker


*/
pub fn make_brach(x:f64, y:f64, w:f64, h:f64, color:&str, pose:Pose, left:bool) -> Group {
    let spacer:f64 = h / TWELFTH;
    let brach_h:f64 = get_arm_length(h);
    let brach_w:f64 = get_arm_width(w);
    let spacer:f64 = get_deltoid_spacer(w);
    let brach_x:f64;
    if left {
        brach_x = x;
    } else {
        brach_x = x + w - brach_w;
    }
    Group::new()
          .add(make_ellipse(brach_x, y + get_double(spacer), brach_w, brach_h, color))
}

/*
# Hand  maker


*/
pub fn make_(x:f64, y:f64, w:f64, h:f64, color:&str, pose:Pose, left:bool) -> Group {
    Group::new()
          .add(make_ellipse(x, y, w, h, color))
}
