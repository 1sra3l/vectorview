use crate::enums::{Teeth, Expression};
use crate::body::VectorView;
use svg::node::element::Group;

impl VectorView for Expression{
    fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Group {
        match *self {
            Expression::Smile =>self.make_simple_body(x,y,w,h,color.clone(),"#00004a","orange", "#76612d"),
            _=> self.make_simple_body(x,y,w,h,color.clone(),"brown","purple", "green"),
        }
    }
    //fn make_mouth(&self, x:f64, y:f64, w:f64, h:f64, color:&str) -> Group {}
}
