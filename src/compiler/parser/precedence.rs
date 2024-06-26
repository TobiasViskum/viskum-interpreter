#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    PrecNone = 0,
    PrecAssignment = 1,
    PrecOr = 2,
    PrecAnd = 3,
    PrecEquality = 4,
    PrecComparison = 5,
    PrecTerm = 6,
    PrecFactor = 7,
    PrecUnary = 8,
    PrecCall = 9,
    PrecPrimary = 10,
}
impl From<usize> for Precedence {
    fn from(value: usize) -> Self {
        match value {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecOr,
            3 => Precedence::PrecAnd,
            4 => Precedence::PrecEquality,
            5 => Precedence::PrecComparison,
            6 => Precedence::PrecTerm,
            7 => Precedence::PrecFactor,
            8 => Precedence::PrecUnary,
            9 => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _ => panic!("Invalid precedence value: {}", value),
        }
    }
}
impl From<Precedence> for usize {
    fn from(value: Precedence) -> Self {
        match value {
            Precedence::PrecNone => 0,
            Precedence::PrecAssignment => 1,
            Precedence::PrecOr => 2,
            Precedence::PrecAnd => 3,
            Precedence::PrecEquality => 4,
            Precedence::PrecComparison => 5,
            Precedence::PrecTerm => 6,
            Precedence::PrecFactor => 7,
            Precedence::PrecUnary => 8,
            Precedence::PrecCall => 9,
            Precedence::PrecPrimary => 10,
        }
    }
}
impl Precedence {
    pub fn get_next(self) -> Self {
        if self == Precedence::PrecPrimary {
            Precedence::PrecNone
        } else {
            Precedence::from((self as usize) + 1)
        }
    }
    pub fn get_previous(self) -> Self {
        if self == Precedence::PrecNone {
            Precedence::PrecPrimary
        } else {
            Precedence::from((self as usize) - 1)
        }
    }
}
