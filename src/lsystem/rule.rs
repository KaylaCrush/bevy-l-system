/// A rule in an L-System
#[derive(Clone)]
pub struct Rule {
    pub predecessor: char,      // For now, single char
    pub successor: String,      // Replacement string
    pub probability: f32,       // 0.0..1.0
}

impl Rule {
    pub fn new(predecessor: char, successor: &str) -> Self {
        Self {
            predecessor,
            successor: successor.to_string(),
            probability: 1.0,
        }
    }

    pub fn with_probability(predecessor: char, successor: &str, probability: f32) -> Self {
        Self {
            predecessor,
            successor: successor.to_string(),
            probability,
        }
    }
}
