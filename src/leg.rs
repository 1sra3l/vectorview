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
    make_leg_armor(x, y, w, h, color, color, color, color, left)
}
/*
#  maker


*/
pub fn make_leg_armor(x:f64, y:f64, w:f64, h:f64, quad_color:&str, soleus_color:&str, knee_color:&str, under_color:&str, left:bool) -> Group {
    let y = get_hip_y(y, h);
    //let w = get_leg_width(w);
    //let h = get_leg_length(h);
    //let x = get_leg_x(x, w, left);
    let pose = Pose::None;
    let quad = make_quad(x, y, w, h, quad_color, under_color, pose, left);
    let knee = make_knee(x, y, w, h, knee_color, pose, left);
    let soleus = make_soleus(x, y, w, h, soleus_color, under_color, pose, left);
    Group::new()
          .add(quad)
          .add(soleus)
          .add(knee)
}
/*
# Quadriceps femoris


*/
pub fn make_quad(x:f64, y:f64, w:f64, h:f64, color:&str, under_color:&str, pose:Pose, left:bool) -> Group {
    let new_h:f64 = get_leg_length(h);
    let new_w:f64 = get_leg_width(w);
    let spacer:f64 = get_deltoid_spacer(w);
    let under_w:f64 = get_half(new_w);
    let half_diff = get_half(new_w - under_w);
    //let x = get_leg_x(x, w, left);
    let new_x:f64;
    if left {
        new_x = get_trunk_x(x, w);
    } else {
        new_x = get_trunk_x(x, w) + get_trunk_width(w) - new_w
    }
    let under_x:f64 = new_x + half_diff;
    Group::new()
          .add(make_ellipse(new_x, y + get_double(spacer), new_w, new_h, color))
          .add(make_rectangle_roundness(under_x, y + get_double(spacer), under_w, new_h, under_color, ROUNDNESS))
}

/*
# Quadriceps femoris


*/
pub fn make_knee(x:f64, y:f64, w:f64, h:f64, color:&str, pose:Pose, left:bool) -> Group {
    let new_w:f64 = get_leg_width(w);
    let spacer:f64 = get_deltoid_spacer(w);
    let new_x:f64;
    let half:f64 = get_half(new_w);
    let y = y + get_leg_length(h) + half;
    if left {
        new_x = get_trunk_x(x, w);
    } else {
        new_x = get_trunk_x(x, w) + get_trunk_width(w) - new_w;
    }
    Group::new()
          .add(make_ellipse(new_x, y, half, new_w, color))
}
/*
# Soleus maker


*/
pub fn make_soleus(x:f64, y:f64, w:f64, h:f64, color:&str, under_color:&str, pose:Pose, left:bool) -> Group {
    let new_h:f64 = get_leg_length(h);
    let new_w:f64 = get_leg_width(w);
    let spacer:f64 = get_deltoid_spacer(w);
    let under_w:f64 = get_half(new_w);
    let half_diff = get_half(new_w - under_w);
    let y = y + new_h;
    let calf_h:f64 = new_h - (new_h / 3.0);
    let new_x:f64;
    if left {
        new_x = get_trunk_x(x, w);
    } else {
        new_x = get_trunk_x(x, w) + get_trunk_width(w) - new_w;
    }
    let under_x:f64 = new_x + half_diff;
    Group::new()
          .add(make_ellipse(new_x, y + get_double(spacer), new_w, calf_h, color))
          .add(make_rectangle_roundness(under_x, y + get_double(spacer), under_w, new_h, under_color, ROUNDNESS))
}
