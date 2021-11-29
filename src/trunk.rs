/*!
# Trunk


*/
use svg::node::element::Group;
use crate::utils::*;

pub fn make_trunk(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    let pecs = make_pecs(x, y, w, h, color);
    let abs = make_abs(x, y, w, h, color);
    let delt_r = make_deltoid(x, y, w, h, color, false);
    let delt_l = make_deltoid(x, y, w, h, color, true);
    let debug = make_rectangle(x, y, w, h, "red");
    Group::new()
          //.add(debug)
          .add(pecs)
          .add(delt_r)
          .add(delt_l)
          .add(abs)
}
/*
# Deltoid maker


*/
pub fn make_deltoid(x:f64, y:f64, w:f64, h:f64, color:&str, left:bool) -> Group {
    let deltoid_spacer:f64 = w / TWELFTH;
    let deltoid_h:f64 = h / QUARTER_DIVISOR;
    let deltoid_w:f64 = w / QUARTER_DIVISOR;
    let deltoid_x:f64;
    if left {
        deltoid_x = x + w - deltoid_w;
    } else {
        deltoid_x = x;
    }
    Group::new()
          .add(make_ellipse(deltoid_x, y + deltoid_spacer, deltoid_w, deltoid_h, color))
}
/*
# Rectus abdominis + abdominal external oblique maker


*/
pub fn make_abs(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    let y_spacer = h / TWELFTH;
    let deltoid_spacer:f64 = w / TWELFTH;
    let offset = 2.0 * deltoid_spacer;
    let center_x:f64 = x + (w / HALF_DIVISOR);
    let pec_h:f64  = h  / HALF_DIVISOR;
    let abs_h:f64 = h - pec_h + y_spacer;
    let abs_y:f64 = y + pec_h - y_spacer;
    let abs_w:f64 = w - (w / THIRD_DIVISOR);
    let abs_x:f64 = center_x - (abs_w / HALF_DIVISOR) + deltoid_spacer;
    let obl_w:f64 = w - (w / 9.0);
    let obl_x:f64 = center_x - (obl_w / HALF_DIVISOR) + deltoid_spacer;
    let obl_h:f64 = h - (h / QUARTER_DIVISOR);
    let abs = make_rectangle_roundness(abs_x, abs_y, abs_w- offset, abs_h, color, 7.0);
    let obliq = make_ellipse(obl_x, y, obl_w- offset, obl_h, color);
    Group::new()
          .add(abs)
          .add(obliq)
}
/*
# Pectoralis major maker


*/
pub fn make_pecs(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    let pec_h:f64  = h  / HALF_DIVISOR;
    let deltoid_spacer:f64 = w / TWELFTH;
    let offset = 2.0 * deltoid_spacer;
    Group::new()
          .add(make_ellipse(x + deltoid_spacer, y, w - offset, pec_h, color))
}
