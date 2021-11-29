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
