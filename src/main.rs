use tms_rs::machine::*;
use tms_rs::macros::*;

fn print_state(tm: &TuringMachine) {
    let (output, head_idx) = tm.read_tape();
    let state = tm.state().to_string();
    println!(
        "|{}| {}",
        state,
        output
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let mut padding = output[..=head_idx]
        .iter()
        .fold(state.len() + 3, |acc, s| match s {
            Symbol::AlphabetSymbol(_) => acc + 3, // '0'
            Symbol::Blank => acc + 1,             // □
        })
        + head_idx; // blank spaces

    if let Symbol::AlphabetSymbol(_) = output[head_idx] {
        padding -= 1; // center
    }

    println!("{:>width$}", "↑", width = padding);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // converts ones to zeroes
    // https://www.cl.cam.ac.uk/projects/raspberrypi/tutorials/turing-machine/one.html
    let mut tm = TuringMachine::new(
        alph!['0', '1'],
        states![s0, s1, stop],
        states!(s0),
        input!['1', '1', '0'],
        states![stop,],
        transitions![
            (s0, '0') -> (s0, '1', >),
            (s0, '1') -> (s0, '0', >),
            (s0, _) -> (s1, _, <),
            (s1, '0') -> (s1, '1', <),
            (s1, '1') -> (s1, '0', <),
            (s1, _) -> (stop, _, .),
        ],
    )?;

    print_state(&tm);
    while let Ok(res) = tm.step() {
        print_state(&tm);
        match res {
            StepResult::Accept => {
                println!("Accepted!");
                break;
            }
            StepResult::Reject => {
                println!("Rejected!");
                break;
            }
            StepResult::Continue => {}
        }
    }

    Ok(())
}
