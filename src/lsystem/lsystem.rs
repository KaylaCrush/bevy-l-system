use crate::lsystem::rule::Rule;

/// Pure, stateless L-System blueprint
pub struct LSystem {
    pub axiom: String,
    pub rules: Vec<Rule>,
    pub angle: f32
}

impl LSystem {
    pub fn new(axiom: &str, rules: Vec<Rule>, angle: f32) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules,
            angle,
        }
    }
}
