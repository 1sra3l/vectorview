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
pub mod body;
pub mod prelude;
pub mod utils;
pub mod teeth;
pub mod nose;
pub mod ear;
pub mod eye;
pub mod head;
pub mod mouth;
pub mod icon;
pub mod trunk;
pub mod arm;
pub mod leg;
pub mod calculator;

#[cfg(feature = "rpg")]
pub mod rpgstat;

#[cfg(test)]
mod tests {
    use crate::body::VectorView;
    use crate::prelude::*;
    use svg::Document;
    use crate::enums::{Expression, Teeth};
    use svg::node::element::Group;
    #[cfg(feature = "rpg")]
    use crate::rpgstat::*;
    #[cfg(feature = "rpg")]
    use rpgstat::effect::Normal as NormalEffect;
    #[cfg(feature = "rpg")]
    use rpgstat::types::Advanced as AdvancedType;
    
    pub struct Test(u32);
    impl VectorView for Test {
        fn make_image(&self, x:f64, y:f64, w:f64, h:f64, color:&str, opacity:f64) -> Group {
            self.make_face(x, y, w, h, color, "#662680", "#bd601e",  "#422808", "#ffffc4", false, Teeth::Straight)
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
    #[cfg(feature = "rpg")]
    #[test]
    fn test_animal() {
        let x:f64 = 0.0;
        let y:f64 = 0.0;
        let w:f64 = 64.0;
        let h:f64 = 128.0;
        let face = AdvancedType::None;
        let document = Document::new()
            .set("viewBox", (x, y, w, h))
            .add(face.make_image(x, y, w, h, "#8c7436", 1.0));
        svg::save("rpgstat-animal-image.svg", &document).unwrap();
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
    #[cfg(feature = "rpg")]
    #[test]
    fn test_rpgstat_effect_normal_icon() {
        let x:f64 = 0.0;
        let y:f64 = 0.0;
        let w:f64 = 32.0;
        let h:f64 = 32.0;
        let effect = NormalEffect::Poison;
        let document = Document::new()
            .set("viewBox", (x, y, w, h))
            .add(effect.make_image(x, y, w, h, "red", 1.0)); // red background
        svg::save("rpgstat-poison.svg", &document).unwrap();
    }

}
