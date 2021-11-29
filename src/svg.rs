use std::fmt;
use svg::Document;
use svg::node::element::{Ellipse, Filter, Rectangle, path::Data, Path};
use svg::node::element::Group;
use crate::enums::Teeth;
use crate::utils::*;
use crate::teeth::*;
use crate::nose::*;
use crate::eye::*;
use crate::ear::*;
/*
This trait builds vector graphics for characters and creatures
*/
pub trait VectorView {
/*
This function is the one you define what is drawn
Choose from any of the functions in this trait to compose your object visually
*/
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Group;
/*
Draw an eye uses default method unless overridden
*/
    fn make_eye(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
        make_eye_default(x,y,w,h,color.clone())
    }
/*
Make ear uses default method unless overridden
*/
    fn make_ear(&self, x:f64, y:f64, w:f64, h:f64, ear_color:&str, left:bool) -> Group {
        make_ear_default(x,y,w,h,ear_color,left)
    }
/*
Make teeth, override for different teeth
*/
    fn make_teeth(&self, x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
        make_teeth_default(x, y, w, h, color, teeth)
    }
    /*
Make nose uses default method unless overridden
*/
    fn make_nose(&self, x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
        make_nose_default(x,y,w,h,nose_color)
    }
/*
Draw a face uses default method unless overridden
*/
    fn make_face(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str,  anger:bool, teeth:Teeth) -> Group {
        self.make_face_default(x, y, w, h, skin_color, eye_color, nose_color, hair_color, teeth_color, false, teeth)
    }
/*
Make an arm uses default method unless overridden
*/
    fn make_arm(&self, x:f64, y:f64, length:f64, thickness:f64, clothes_color:&str, left:bool) -> Group {
        self.make_arm_default(x, y, length, thickness, clothes_color, left)
    }
/*
The fully configurable body uses default method unless overridden.
*/
    fn make_full_body(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str, clothes_color1:&str, clothes_color2:&str, teeth:Teeth) -> Group {
        self.make_full_body_default(x, y, w, h, skin_color, eye_color, nose_color, hair_color, teeth_color, clothes_color1, clothes_color2, teeth)
    }
/*
The four color body (teeth are always white) uses default method unless overridden
*/
    fn make_simple_body(&self, x:f64, y:f64, w:f64, h:f64, color1:&str, color2:&str, color3:&str, color4:&str) -> Group {
        self.make_simple_body_default(x, y, w, h, color1, color2, color3, color4, Teeth::Straight)
    }
/*
Make nose uses default method unless overridden
*/
    fn make_animal_nose(&self, x:f64, y:f64, w:f64, h:f64, nose_color:&str) -> Group {
        make_animal_nose_default(x, y, w, h, nose_color)
    }
/*
Make skull uses default method unless overridden
*/
    fn make_skull(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
        self.make_skull_default(x, y, w, h, color)
    }
/*
Make mouth uses default method unless overridden
*/
    fn make_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
        self.make_smile_mouth(x,y,w,h,color.clone(), teeth)
    }

//----------------------------------------------------------------------------------------------------------------------------------------------
// # DEFAULT FUNCTIONS
//----------------------------------------------------------------------------------------------------------------------------------------------
/*
The four color body (teeth are always white)
*/
    fn make_simple_body_default(&self, x:f64, y:f64, w:f64, h:f64, color1:&str, color2:&str, color3:&str, color4:&str, teeth:Teeth) -> Group {
        let skin = color1;
        let nose = color4;//TODO tint color
        let eye = color2;
        let hair = color3;
        let teeth_color = "white";
        let clothes1 = color3;
        let clothes2 = color2;
        self.make_full_body(x, y, w, h, skin, eye, nose, hair, teeth_color, clothes1, clothes2, teeth)
    }
/*
Make an arm
*/
    fn make_arm_default(&self, x:f64, y:f64, length:f64, thickness:f64, clothes_color:&str, left:bool) -> Group {
        //let spacer = (length / thickness) / 2.0;
        let arm_upper = make_rectangle(x, y, length, thickness, clothes_color);
        let arm_x:f64;
        if left {
            arm_x = x;
        } else {
            arm_x = x + length - thickness;
        }
        let arm = make_rectangle(arm_x, y, thickness, length, clothes_color);
        Group::new()
                 .add(arm_upper)
                 .add(arm)
    }
/*
The fully configurable body.
*/
    fn make_full_body_default(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str, clothes_color1:&str, clothes_color2:&str,teeth:Teeth) -> Group {
        let half_w:f64 = w / 2.0;
        let center_x:f64 = x + half_w;
        let quarter_w:f64 = w / 4.0;
        let head_h:f64 = h / 6.5;
        let head_w:f64 = quarter_w;
        let spacer:f64 = head_h / 4.0;
        // center in image based on widths
        let head_x:f64 = ((x + w) / 2.0) - (head_w / 2.0);
        let torso_h:f64 = head_h * 2.0;
        let torso_w:f64 = half_w;
        let arm_length:f64 = quarter_w;
        let arm_thickness:f64 = torso_h / 5.0;
        let leg_thickness:f64 = torso_w / 3.0;
        let leg_length:f64 = arm_length * 2.0;
        let hip_h:f64 = head_h;
        let leg_y:f64 = y + torso_h + hip_h;

        // make the groups
        let debug = make_rectangle(x, y, w, h, "red");
        let head = self.make_face(head_x, y, head_w, head_h, skin_color, eye_color, nose_color, hair_color, teeth_color, false, teeth);
        let hand = skin_color;
        let y = (y + head_h) - spacer;
        
        let clothes1 = make_rectangle(center_x - quarter_w, y, torso_w, torso_h, clothes_color1);
        let l_arm = self.make_arm(x, y, arm_length, arm_thickness, clothes_color1, true);
        let r_arm = self.make_arm(center_x + (torso_w / 2.0), y, arm_length, arm_thickness, clothes_color1, false);
        let hips = make_rectangle(center_x - quarter_w, y + torso_h - spacer, torso_w, hip_h, clothes_color2);
        let leg1 = make_rectangle(center_x - quarter_w, leg_y, leg_thickness, leg_length, clothes_color2);
        let leg2 = make_rectangle(center_x + quarter_w - leg_thickness, leg_y, leg_thickness, leg_length, clothes_color2);
        Group::new()
                 .add(debug)
                 .add(hips)
                 .add(leg1)
                 .add(leg2)
                 .add(clothes1)
                 .add(r_arm)
                 .add(l_arm)
                 .add(head)
    }

/*
Make Head
*/
    fn make_head(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str) -> Group {
    //face
        
        let center_x:f64 = x + (w / 2.0);
        let chin:f64 = h / 4.0;
        let face_h:f64 = h - chin;
        let jaw_h:f64 = h - face_h;
        let jaw_w:f64 = (w / 7.0) * 3.0;
        let jaw_y:f64 = y + h - chin;
        let cheek_w:f64 = (w / 5.0) * 4.0;
        let jaw_x:f64 = x + ((w - jaw_w) / 2.0);
        let face = make_ellipse(x, y, w, face_h, skin_color);
        let cheeks = make_ellipse(center_x - (cheek_w / 2.0), y, cheek_w, h, skin_color);
        let jaw = make_rectangle_roundness(jaw_x, jaw_y, jaw_w, jaw_h, skin_color, 7.0);
        Group::new()
              .add(cheeks)
              .add(face)
              .add(jaw)
    }

/*
Make full face
*/
    fn make_face_default(&self, x:f64, y:f64, w:f64, h:f64, skin_color:&str, eye_color:&str, nose_color:&str, hair_color:&str, teeth_color:&str, anger:bool, teeth:Teeth) -> Group {
        let ear_w:f64 = w / 6.0;
        let spacer = w / 15.0;
        let hair_h:f64 = 0.0;//h / 10.0;
        let face_w:f64 = w - (ear_w * 2.0) + (spacer * 2.0);
        let ear_lx:f64 = x + 0.0;
        let face_x:f64 = ear_lx + ear_w - spacer;
        let face_y:f64 = y + hair_h;
        let total_face_h:f64 = h - hair_h;
        let face_h:f64 = total_face_h - (total_face_h / 4.0);
        let features_y = face_y + (face_h / 2.0);
        let eye_w:f64 = face_w / 5.0;
        let jaw_w:f64 = face_w / 3.0;//(face_w / 16.0) * 8.5;

        let half_eye:f64 = eye_w / 2.0;
        let brow_h:f64 = eye_w / 8.0;
        let brow_y:f64 = features_y - brow_h;
        let nose_w:f64 = half_eye;
        let half_nose:f64 = nose_w / 2.0;
        let nose_h:f64 = eye_w * 2.0;
        let ear_h:f64 = nose_h;
        let ear_rx:f64 = face_x + face_w - spacer;
        let half:f64 = w / 2.0;
        let nose_x:f64 = x + half - half_nose;
        let center_x:f64 = x + half;

        let cheek_w:f64 = (w / 5.0) * 4.0;
        let mouth_w:f64 = cheek_w / 2.0;
        let mouth_x:f64 = center_x - (mouth_w / 2.0);
        let mouth_h:f64 = total_face_h / 5.0;
        let nose_spacer:f64 = h / 20.0;
        let mouth_y:f64 = features_y + nose_h + nose_spacer;
        let eye_positioner:f64 = eye_w + (eye_w / 2.0);
        let eye_l_x:f64 = center_x - eye_positioner;
        
        let bg = make_rectangle(x,y,w,h, "red");
        //face
        let face = self.make_head(face_x, face_y, face_w, total_face_h, skin_color);
        //nose
        let nose = self.make_nose(nose_x, features_y, nose_w, nose_h, nose_color);
        //ears
        let l_ear = self.make_ear(ear_lx, features_y, ear_w, eye_w * 2.0, skin_color, true);
        let r_ear = self.make_ear(face_x + face_w - spacer, features_y, ear_w, eye_w * 2.0, skin_color, false);
        //eyebrows
        let eye_brow_r = make_slant(eye_l_x, brow_y, eye_w, brow_h, hair_color, 1.0, !anger);
        let eye_brow_l = make_slant(center_x + half_eye, brow_y, eye_w, brow_h, hair_color, 1.0, anger);
        //eyes
        let eye_l = self.make_eye(eye_l_x, features_y, eye_w, eye_w, eye_color);
        let eye_r = self.make_eye(center_x + half_eye, features_y, eye_w, eye_w, eye_color);
        //mouth
        let mouth = self.make_mouth(mouth_x, mouth_y, mouth_w, mouth_h, teeth_color, teeth);
        //let hair = self.make_hair(x,y,w,h,hair_color);
        Group::new()
                    .add(bg)
                    .add(face)
                    .add(l_ear)
                    .add(r_ear)
                    .add(nose)
                    .add(eye_r)
                    .add(eye_l)
                    .add(eye_brow_r)
                    .add(eye_brow_l)
                    .add(mouth)
                    //.add(hair)
    }

/*
Make skull
*/
    fn make_skull_default(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {
        //let spacer = w / 24.0;
        let face_w = w;
        let eye_w:f64 = face_w / 5.0;
        let half_eye:f64 = eye_w / 2.0;
        let face_x = x;
        let face_h = h - (h / 3.0);
        let features_y = y + (h / 3.0);
        let brow_h:f64 = eye_w / 8.0;
        let brow_y:f64 = features_y - brow_h;
        let nose_w:f64 = half_eye;
        let half:f64 = face_w / 2.0;
        let nose_x:f64 = half + face_x - (nose_w / 2.0);
        let center_x:f64 = nose_x - (nose_w / 2.0);
        let mouth_y:f64 = face_x + face_h - nose_w;
        let mouth_w:f64 = eye_w * 3.0;
        let mouth_h:f64 = face_h / 3.0;
        let eye_l_x:f64 = center_x - eye_w;
        let mouth_x:f64 = eye_l_x;//center_x - (mouth_w / 4.0);
        //face
        let face = make_ellipse(face_x, y, face_w, face_h, color);
        //nose
        let nose = make_ellipse_shadow(nose_x, features_y + nose_w, nose_w, eye_w, "black");
        //eyebrows
        let eye_brow_r = make_slant(eye_l_x, brow_y, eye_w, brow_h, "black", 0.4, true);
        let eye_brow_l = make_slant(center_x + eye_w, brow_y, eye_w, brow_h, "black", 0.4, false);
        //eyes
        let eye_l = make_ellipse_shadow(eye_l_x, features_y, eye_w, eye_w, "black");
        let eye_r = make_ellipse_shadow(center_x + eye_w, features_y, eye_w, eye_w, "black");
        //mouth2
        let mouth2 = make_rectangle_roundness(mouth_x, mouth_y, mouth_w, mouth_h / 2.0, color, 4.0);
        //mouth
        let mouth = make_tooth_row(mouth_x, mouth_y, mouth_w, mouth_h, color, 0.7, 5);
        
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


/*
Make the default "smile" mouth
*/
    fn make_smile_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
        let path = make_down_part(x, y, w, h, "black", 1.0);
        let spacer = w / 24.0;
        let tooth_w:f64 = w - (2.0 * spacer);
        let tooth_h:f64 = h / 4.0;
        let tooth = self.make_teeth(x + spacer, y, tooth_w, tooth_h, color, teeth);
        Group::new()
                 .add(path)
                 .add(tooth)
    }
    


}
