use std::collections::HashMap;
use std::fmt;

use crate::errors::*;
use crate::tape::DoublyInfiniteTape;

// use anyhow::Result;

#[derive(Debug)]
pub struct State {
    id: StateID,
    transitions: HashMap<Symbol, Transition>, // TODO: for NDTMs, use `Vec<Transition>` as V
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    AlphabetSymbol(char),
    Blank,
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blank => "□".to_owned(),
                Self::AlphabetSymbol(c) => format!("'{}'", c),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateID(String);
impl fmt::Display for StateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum Movement {
    Left,
    Right,
    Stop,
}

#[derive(Debug)]
pub struct Transition {
    read: Symbol,
    write: Symbol,
    movement: Movement,
    state: StateID,
}

#[derive(Debug)]
pub struct TuringMachine {
    alphabet: Vec<Symbol>,
    states: HashMap<StateID, State>,
    pub current_state: StateID,
    tape: DoublyInfiniteTape,
    initial_state: StateID,
    accepting_states: Vec<StateID>,
}

pub enum StepResult {
    Continue,
    Halt,
}

impl TuringMachine {
    pub fn new(
        alphabet: Vec<Symbol>,
        states: Vec<State>,
        initial_state: StateID,
        input: Vec<Symbol>,
        accepting_states: Vec<StateID>,
    ) -> Self {
        Self {
            alphabet,
            states: states
                .into_iter()
                .map(|s| (s.id.clone(), s))
                .collect::<HashMap<_, _>>(),
            current_state: initial_state.clone(),
            tape: DoublyInfiniteTape::new(input),
            initial_state,
            accepting_states,
        }
    }

    pub fn step(&mut self) -> Result<StepResult, TMRuntimeError> {
        let transition = self
            .states
            .get(&self.current_state)
            .ok_or(TMRuntimeError::InvalidState(self.current_state.clone()))?
            .transitions
            .get(self.tape.peek())
            .ok_or({
                TMRuntimeError::InvalidTransition(
                    self.current_state.clone(),
                    self.tape.peek().clone(),
                )
            })?;

        self.tape.write(transition.write.clone());
        self.tape.r#move(transition.movement.clone());
        self.current_state = transition.state.clone();

        if self.accepting_states.contains(&self.current_state) {
            return Ok(StepResult::Halt);
        }

        Ok(StepResult::Continue)
    }

    pub fn reset(&mut self, input: Vec<Symbol>) {
        self.current_state = self.initial_state.clone();
        self.tape = DoublyInfiniteTape::new(input);
    }
}
