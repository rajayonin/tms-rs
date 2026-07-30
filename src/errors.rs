use std::{collections::HashSet, fmt};

use crate::machine::{StateID, StateSet, Symbol};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TMRuntimeError {
    #[error("Invalid state {0}")]
    InvalidState(StateID),
    #[error("Symbol {0} not in alphabet")]
    SymbolNotInAlphabet(Symbol),
}

#[derive(Debug)]
pub struct InputSet(pub HashSet<Symbol>);

impl fmt::Display for InputSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Error, Debug)]
pub enum TMDefinitionError {
    #[error("Alphabet is missing the blank transition")]
    MissingBlank,
    #[error("Invalid states in transition table: {0}")]
    InvalidStates(StateSet),
    #[error("Invalid inputs: {0}")]
    InvalidInput(InputSet),
}
