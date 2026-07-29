use crate::machine::{StateID, Symbol};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TMRuntimeError {
    #[error("Invalid state {0}")]
    InvalidState(StateID),
    #[error("Symbol {0} not in alphabet")]
    SymbolNotInAlphabet(Symbol),
    #[error("Invalid transition from state {0} with symbol {1}")]
    InvalidTransition(StateID, Symbol),
}
