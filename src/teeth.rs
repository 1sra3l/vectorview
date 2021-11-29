/*!
# Teeth

*/
use svg::node::element::{Ellipse, Rectangle, path::Data, Path};
use svg::node::element::Group;

use crate::enums::Teeth;
use crate::utils::*;

/*
Make Teeth default
*/
pub fn make_teeth_default(x:f64, y:f64, w:f64, h:f64, color:&str, teeth:Teeth) -> Group {
    match teeth {
        Teeth::Straight => {
            let tooth = make_rectangle_roundness(x, y, w, h, color, 0.4);
            return Group::new().add(tooth)
        },
        Teeth::Sharp => {
            let tooth = make_sharp_row(x, y, w, h, color, 1.0, 7);
            return Group::new().add(tooth)
        },//Teeth::Canine => {}, Teeth::Beak => {},
        _=>{
            let tooth = make_rectangle_roundness(x, y, w, h, color, 0.4);
            return Group::new().add(tooth)
        },
    }
}
/*
Get a single flat tooth
*/
pub fn get_tooth(x:f64, y:f64, w:f64, h:f64, color:&str, roundness:f64, amount:usize, current:usize) -> Rectangle {
    let amount:f64 = amount as f64;
    let current:f64 = current as f64;

    let spacer:f64 = (w / amount) / 12.0;
    let width:f64 = (w - (spacer * (amount - 1.0))) / amount;
    let new_w:f64 = width - spacer;
    let spacers:f64 = spacer * current;
    let new_x:f64;
    if current > 0.0 {
        new_x = x + (width * current) + spacers;// + (spacer * 2.0);
    } else {
        new_x = x;// + (spacer / 2.0);
    }
    
    make_rectangle_roundness(new_x,y,new_w,h,color,roundness)
}
/*
Make row of teeth
*/
pub fn make_tooth_row(x:f64, y:f64, w:f64, h:f64, color:&str, roundness:f64, amount:usize) -> Group {
    let mut retval = Group::new();
    for current in 0..amount {
        retval = retval.clone()
        .add(get_tooth(x,y,w,h,color,roundness,amount,current));
    }
    retval
}
/*
Get single fang
*/
pub fn get_fang(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64, amount:usize, current:usize) -> Path {
    let amount:f64 = amount as f64;
    let current:f64 = current as f64;

    let width:f64 = w / amount;
    let spacer:f64 = width / 12.0;
    let new_w:f64 = width - spacer;
    let spacers = spacer * amount;
    
    let new_x:f64 = x + (width * current) + spacers;
    
    make_sharp(new_x,y,new_w,color,opacity)
}
/*
Make row of sharp teeth
*/
pub fn make_sharp_row(x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64, amount:usize) -> Group {
    let mut retval = Group::new();
    for current in 0..amount {
        retval = retval.clone()
        .add(get_fang(x,y,w,h,color,opacity,amount,current));
    }
    retval
}
