/*!
# Icon emblems


*/
use svg::node::element::Group;
use crate::utils::*;
use crate::teeth::*;
/*
Make skull
*/
pub fn make_skull(x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
    //let spacer = w / 24.0;
    let eye_w:f64 = w / 5.0;
    let half_eye:f64 = eye_w / 2.0;
    let face_h = h - (h / 5.0);
    let features_y = y + (h / 2.0);
    let brow_h:f64 = eye_w / 8.0;
    let brow_y:f64 = features_y - brow_h;
    let nose_w:f64 = half_eye;
    let half:f64 = w / 2.0;
    let nose_x:f64 = half + x - (nose_w / 2.0);
    let center_x:f64 = nose_x - (nose_w / 2.0);
    let mouth_y:f64 = x + face_h;//;
    let mouth_w:f64 = eye_w * 3.0;
    let mouth_h:f64 = h - face_h;
    let eye_l_x:f64 = center_x - eye_w;
    let mouth2_spacer:f64 = nose_w / 2.0;
    let mouth_x:f64 = eye_l_x;//center_x - (mouth_w / 4.0);
    //face
    let face = make_ellipse(x, y, w, face_h, color);
    //nose
    let nose = make_ellipse_shadow(nose_x, features_y + nose_w, nose_w, eye_w, "black");
    //eyebrows
    let eye_brow_r = make_slant(eye_l_x, brow_y, eye_w, brow_h, "black", 0.4, true);
    let eye_brow_l = make_slant(center_x + eye_w, brow_y, eye_w, brow_h, "black", 0.4, false);
    //eyes
    let eye_l = make_ellipse_shadow(eye_l_x, features_y, eye_w, eye_w, "black");
    let eye_r = make_ellipse_shadow(center_x + eye_w, features_y, eye_w, eye_w, "black");
    //mouth2
    let mouth2 = make_rectangle_roundness(mouth_x - mouth2_spacer, mouth_y - (2.5 * mouth2_spacer), mouth_w + (2.0 * mouth2_spacer), mouth_h, color, 2.0);
    //mouth
    let mouth = make_tooth_row(mouth_x, mouth_y, mouth_w, mouth_h, color, 0.7, 6);
    
    Group::new()
            .add(face)
            .add(mouth)
            .add(mouth2)
            .add(nose)
            .add(eye_r)
            .add(eye_l)
            .add(eye_brow_r)
            .add(eye_brow_l)
            
}
