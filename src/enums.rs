/*!
# Enums

*/

/*
# Teeth


*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Teeth {
    Straight,
    Sharp,
    Canine,
    Rodent,
    Beak,
    None,
}

/*
# Expression
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Smile,
    Hurt,
    Angry,
    None,
}

/*
# Pose
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pose {
    Fight,
    Defend,
    Hurt,
    Tired,
    Excited,
    None,
}

/*
# 
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Person,
    Rodent,
    Icon,
    None,
}
