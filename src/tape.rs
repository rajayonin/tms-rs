use crate::machine::{Movement, Symbol};

#[derive(Debug, Clone)]
struct Cell(Symbol);

type CellIndex = i32;

/// An infinite, double-sided tape, filled with `Symbol::Blank`.
#[derive(Debug)]
pub struct DoublyInfiniteTape {
    // to make it infinite and double-sided, without using mmap, we use a vector
    // for the "positive values" and another for the "negative values"
    positive_tape: Vec<Cell>,
    negative_tape: Vec<Cell>,
    head: CellIndex,
}

impl DoublyInfiniteTape {
    /// Creates a new tape.
    ///
    /// Sets the head to the last symbol.
    pub fn new(input: Vec<Symbol>) -> Self {
        Self {
            head: input.len() as i32 - 1,
            positive_tape: input.into_iter().map(|s| Cell(s)).collect(),
            negative_tape: Vec::new(),
        }
    }

    /// Moves the head with the specified movement.
    pub fn r#move(&mut self, movement: Movement) {
        match movement {
            Movement::Left => {
                if self.head <= 0 && (self.head.abs() as usize >= self.negative_tape.len()) {
                    self.negative_tape.push(Cell(Symbol::Blank));
                }
                self.head -= 1;
            }

            Movement::Right => {
                if self.head >= 0 && (self.head as usize >= self.positive_tape.len() - 1) {
                    // new empty cell
                    self.positive_tape.push(Cell(Symbol::Blank));
                }
                self.head += 1
            }

            Movement::Stop => {}
        }
    }

    /// Shows the symbol under the head.
    pub fn peek(&self) -> &Symbol {
        match self.head {
            ..0 => &self.negative_tape[(self.head.abs() - 1) as usize].0,
            _ => &self.positive_tape[self.head as usize].0,
        }
    }

    /// Writes the specified symbol at the head.
    pub fn write(&mut self, value: Symbol) {
        match self.head {
            ..0 => self.negative_tape[(self.head.abs() - 1) as usize] = Cell(value),
            _ => self.positive_tape[self.head as usize] = Cell(value),
        }
    }

    /// Returns the filled tape and the head position.
    pub fn output(&self) -> (Vec<&Symbol>, usize) {
        // reverse negative tape
        let mut output_tape: Vec<_> = self.negative_tape.iter().map(|c| &c.0).rev().collect();

        // add positive tape
        output_tape.extend(self.positive_tape.iter().map(|c| &c.0));

        (
            output_tape,
            (self.negative_tape.len() as CellIndex + self.head) as usize,
        )
    }
}
