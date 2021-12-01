/*!
# Calculator

Body proportion math

This module has handy functions like:
```
use vectorview::calculator::*;
let num:f64 = 100.0;
assert_eq!(get_double(num), 200.0);
// center and half are the same
assert_eq!(get_center(num), get_half(num));
// head width is a quarter
assert_eq!(get_quarter(num), get_head_width(num));
//trunk width is half
assert_eq!(get_half(num), get_trunk_width(num));

```

*/
use crate::utils::*;

/*
# Center

This is to make other code more clearly show what it is doing

*/
pub fn get_center(num:f64) -> f64 {
    num / HALF_DIVISOR
}

/*
# Half

This is to make other code more clearly show what it is doing

*/
pub fn get_half(num:f64) -> f64 {
    num / HALF_DIVISOR
}
/*
# Half

This is to make other code more clearly show what it is doing

*/
pub fn get_double(num:f64) -> f64 {
    num * HALF_DIVISOR
}
/*
# Quarter

This is to make other code more clearly show what it is doing

*/
pub fn get_quarter(num:f64) -> f64 {
    num / QUARTER_DIVISOR
}
//------------------------------------------ Head
/*
# Head Height

This calculates the general head to body ratio

*/
//TODO add enum support
pub fn get_head_height(num:f64) -> f64 {
    num / 6.5
}
/*
# Hip Height

This calculates the general head to body ratio

*/
//TODO add enum support
pub fn get_hip_height(num:f64) -> f64 {
    num / 13.0
}
/*
# Head Width

This calculates the general head to body ratio

*/
//TODO add enum support
pub fn get_head_width(num:f64) -> f64 {
    get_quarter(num)
}
/*
# Trunk Height

This calculates the general head to body ratio

*/
pub fn get_trunk_height(num:f64) -> f64 {
    let head_h = get_head_height(num);
    get_double(head_h)
}
/*
# Trunk Width

This calculates the general head to body ratio

*/
pub fn get_trunk_width(num:f64) -> f64 {
    get_center(num)
}
/**/
pub fn get_trunk_y(y:f64, h:f64) -> f64 {
    y + get_head_height(h)
}
/*
# Trunk x coordinate

This wraps other functions into one easy function
*/
pub fn get_trunk_x(x:f64, w:f64) -> f64 {
    let center = x + get_center(w);
    let trunk_w = get_trunk_width(w);
    center - get_center(trunk_w)
}
/*
# Hip y coordinate

This wraps other functions into one easy function
*/
pub fn get_hip_y(y:f64, h:f64) -> f64 {
    let trunk_y = get_trunk_y(y, h);
    let trunk_h = get_trunk_height(h);
    trunk_y + trunk_h
}
/*
# Head x coordinate

This wraps other functions into one easy function

*/
pub fn get_head_x(x:f64, w:f64) -> f64 {
    let center = x + get_center(w);
    let head_w = get_head_width(w);
    center - get_center(head_w)
}
/*
# Leg  Width

This calculates the general head to body ratio
*/
pub fn get_leg_width(num:f64) -> f64 {
    num / 5.0
}
/*
# Leg Length

This calculates the general head to body ratio for a single segment of arm
*/
pub fn get_leg_length(num:f64) -> f64 {
    let trunk = get_trunk_height(num);
    trunk - (trunk /  3.5)
}
/*
# Arm  Width

This calculates the general head to body ratio
*/
pub fn get_arm_width(num:f64) -> f64 {
    num / EIGHT_DIVISOR
}
/*
# Arm Length

This calculates the general head to body ratio for a single segment of arm
*/
pub fn get_arm_length(num:f64) -> f64 {
    let trunk = get_trunk_height(num);
    get_center(trunk)
}
/*
# Arm y coordinate

This wraps other functions into one easy function
*/
pub fn get_arm_y(y:f64, h:f64) -> f64 {
    get_trunk_y(y,h)
}
/*
# Arm x coordinate

This wraps other functions into one easy function

*/
pub fn get_arm_x(x:f64, w:f64, left:bool) -> f64 {
    let center = x + get_center(w);
    let trunk_w = get_trunk_width(w);
    let trunk_center = get_center(trunk_w);
    let x_return:f64;
    if left {
        x_return = center - trunk_center;
    } else {
        x_return = x + w - get_arm_width(w);
    }
    let output = match left {
        true => "Left",
        false => "Right",
    };
    println!("{} x:{}", output, x_return);
    x_return
}
/**/
pub fn get_deltoid_spacer(w:f64) -> f64 {
    get_trunk_width(w) / 10.0
}

/**/
//pub fn get_(x:f64, y:f64, w:f64, h:f64) -> f64 {}
