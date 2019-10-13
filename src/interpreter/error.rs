use crate::common::Annot;
use crate::common::Loc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    TapeBufferOverflow,
    NegativePosition,
    CannotDecodeCharacter,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl InterpreterError {
    pub fn buffer_overflow(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::TapeBufferOverflow, loc)
    }
    pub fn negative_postion(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::NegativePosition, loc)
    }
    pub fn cannot_decode_character(loc: Loc) -> Self {
        Self::new(InterpreterErrorKind::CannotDecodeCharacter, loc)
    }
}