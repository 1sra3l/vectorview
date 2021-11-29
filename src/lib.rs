/*!
# Vector View library

[![Documentation](https://docs.rs/vectorview/badge.svg)](https://docs.rs/vectorview)
[![Crates.io](https://img.shields.io/crates/v/vectorview.svg)](https://crates.io/crates/vectorview)

Cargo.toml
`vectorview="2.0"`

This uses the crate [`svg`](https://docs.rs/svg) to process all SVG related functions

## A General purpose vector art creation library

Initial work:

![Alt text](https://raw.githubusercontent.com/1sra3l/vectorview/main/green.svg?raw=true "Green")

# Modules

These are the different modules containing various shape drawing functions

## Ear

 * Traditional ear

## Eye

 * Traditional eye.
 * Cartoon eye.

## Nose
 * Cartoon muppet nose
 * Traditional nose

## Teeth
 * Traditional flat teeth
 * Sharp teeth row
 * individual flat teeth row


*/
pub mod enums;
pub mod svg;
pub mod prelude;
pub mod utils;
pub mod teeth;
pub mod nose;
pub mod ear;
pub mod eye;
pub mod head;
pub mod mouth;


#[cfg(feature = "rpg")]
pub mod rpgstat;

#[cfg(test)]
mod tests {
    use crate::svg::VectorView;
    use crate::prelude::*;
    use svg::Document;
    use crate::enums::{Expression, Teeth};
    use svg::node::element::Group;
    
    pub struct Test(u32);
    impl VectorView for Test {
        fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Group {
            self.make_face(x, y, w, h, color, "purple", "pink",  "brown", "white", false, Teeth::Straight)
        }
    }
    #[test]
    fn test_vector_expression() {
        let x:f64 = 0.0;
        let y:f64 = 0.0;
        let w:f64 = 64.0;
        let h:f64 = 128.0;
        let face = Expression::Smile; //implements VectorView trait
        let document = Document::new()
            .set("viewBox", (x, y, w, h))
            .add(face.make_image(x, y, w, h, "#8c7436", 1.0));
        svg::save("image.svg", &document).unwrap();
    }
    #[test]
    fn test_vector_effect() {
        let x:f64 = 0.0;
        let y:f64 = 0.0;
        let w:f64 = 80.0;
        let h:f64 = 100.0;
        let effect = Test(0); //implements VectorView trait
        let document = Document::new()
            .set("viewBox", (x, y, w, h))
            .add(effect.make_image(x, y, w, h, "yellow", 1.0));
        svg::save("yellow.svg", &document).unwrap();
        let effect = Test(1);
        let document = Document::new()
            .set("viewBox", (x, y, w, h))
            .add(effect.make_image(x, y, w, h, "green", 1.0));
        svg::save("green.svg", &document).unwrap();
    }

}
