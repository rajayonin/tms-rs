use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::errors::*;
use crate::machine::StepResult::Reject;
use crate::tape::DoublyInfiniteTape;

// use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateID(pub String);
impl fmt::Display for StateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct State {
    // id: StateID,
    transitions: StateTransitions,
}

#[derive(Debug)]
pub struct StateSet(HashSet<StateID>);

impl fmt::Display for StateSet {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// type Alphabet = HashSet<Symbol>;

#[derive(Debug, Clone, PartialEq)]
pub enum Movement {
    Left,
    Right,
    Stop,
}

#[derive(Debug)]
pub struct Transition {
    pub write: Symbol,
    pub movement: Movement,
    pub state: StateID,
}

pub type StateTransitions = HashMap<Symbol, Transition>; // TODO: for NDTMs, use `Vec<Transition>` as V

#[derive(Debug)]
pub struct TuringMachine {
    states: HashMap<StateID, State>,
    current_state: StateID,
    tape: DoublyInfiniteTape,
    initial_state: StateID,
    accepting_states: HashSet<StateID>,
}

pub enum StepResult {
    Continue,
    Accept,
    Reject,
}

impl TuringMachine {
    pub fn new(
        alphabet: HashSet<Symbol>,
        states: HashSet<StateID>,
        initial_state: StateID,
        input: Vec<Symbol>,
        accepting_states: HashSet<StateID>,
        mut transitions: HashMap<StateID, StateTransitions>,
    ) -> Result<Self, TMDefinitionError> {
        if !alphabet.contains(&Symbol::Blank) {
            return Err(TMDefinitionError::MissingBlank);
        }

        // check all transitions are correct
        for (id, state_transitions) in transitions.iter() {
            // all symbols in transition
            // TODO: remove this, if there is no transition, then we stop
            if state_transitions
                .keys()
                .cloned()
                .collect::<HashSet<Symbol>>()
                != alphabet
            {
                return Err(TMDefinitionError::MissingTransition(
                    id.clone(),
                    &alphabet - &state_transitions.keys().cloned().collect(),
                ));
            }

            // transition states are valid
            let transition_states = state_transitions
                .values()
                .map(|t| &t.state)
                .cloned()
                .collect::<HashSet<StateID>>();
            if !transition_states.is_subset(&states) {
                return Err(TMDefinitionError::InvalidStates(StateSet(
                    &transition_states - &states,
                )));
            }
        }

        // ensure all input symbols in alphabet
        let input_set = input.iter().copied().collect::<HashSet<_>>();
        if !input_set.is_subset(&alphabet) {
            return Err(TMDefinitionError::InvalidInput(InputSet(
                &input_set - &alphabet,
            )));
        }

        Ok(Self {
            // alphabet,
            states: states
                .into_iter()
                .map(|id| {
                    (
                        id.clone(),
                        State {
                            // id: id.clone(),
                            transitions: transitions.remove(&id).unwrap_or(HashMap::new()),
                        },
                    )
                })
                .collect(),
            current_state: initial_state.clone(),
            tape: DoublyInfiniteTape::new(input),
            initial_state,
            accepting_states,
        })
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

        // TODO: if no transition, reject

        self.tape.write(transition.write.clone());
        self.tape.r#move(transition.movement.clone());
        self.current_state = transition.state.clone();

        if self.accepting_states.contains(&self.current_state) {
            return Ok(StepResult::Accept);
        }

        if transition.movement == Movement::Stop {
            return Ok(Reject);
        }

        Ok(StepResult::Continue)
    }

    pub fn read_tape(&self) -> (Vec<&Symbol>, usize) {
        self.tape.output()
    }

    pub fn state(&self) -> &StateID {
        &self.current_state
    }

    pub fn reset(&mut self, input: Vec<Symbol>) {
        self.current_state = self.initial_state.clone();
        self.tape = DoublyInfiniteTape::new(input);
    }
}
