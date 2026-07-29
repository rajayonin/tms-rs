#[macro_export]
macro_rules! states {
    ($id:tt) => {
        $crate::machine::StateID(stringify!($id).to_string())
    };

    ($($item:tt),* $(,)?) => {
        ::std::collections::HashSet::from([ $( states!($item) ),* ])
    };
}

// #[macro_export]
// macro_rules! sym {
//     ($val:tt) => {
//         AlphabetSymbol(stringify!($val).chars().next().unwrap())
//     };
// }

#[macro_export]
macro_rules! alph {
    ($val:expr) => {
        $crate::machine::Symbol::AlphabetSymbol($val)
    };

    ($($item:tt),* $(,)?) => {
        ::std::collections::HashSet::from([ $( alph!($item) ),* , $crate::machine::Symbol::Blank])
    };
}

#[macro_export]
macro_rules! input {
    // Rule for the Blank symbol (using underscore)
    (_) => {
        $crate::machine::Symbol::Blank
    };

    // Rule for a single alphabet symbol (handles 0, a, 🦀, etc.)
    ($val:expr) => {
        $crate::machine::Symbol::AlphabetSymbol($val)
    };

    // Rule for a comma-separated list: syms![0, 1, _, 🦀]
    ($($item:tt),* $(,)?) => {
        vec![ $( input!($item) ),* ]
    };
}

#[macro_export]
macro_rules! transitions {
    // movements
    (@dir >) => { $crate::machine::Movement::Right };
    (@dir <) => { $crate::machine::Movement::Left };
    (@dir .) => { $crate::machine::Movement::Stop };

    // symbols
    (@sym _) => { $crate::machine::Symbol::Blank };
    (@sym $sym:expr) => { $crate::machine::Symbol::AlphabetSymbol($sym) };

    // entrypoint
    ( $( ( $s_in:ident, $sym_in:tt ) -> ( $s_out:ident, $sym_out:tt, $move:tt ) ),* $(,)? ) => {{
        let mut _transtion_table = std::collections::HashMap::<$crate::machine::StateID, $crate::machine::StateTransitions>::new();
        $(
            let _state_transitions = _transtion_table
                .entry($crate::machine::StateID(stringify!($s_in).to_string()))
                .or_insert_with(std::collections::HashMap::new);

            _state_transitions.insert(
                transitions!(@sym $sym_in),
                $crate::machine::Transition{
                    write: transitions!(@sym $sym_out),
                    movement: transitions!(@dir $move),
                    state: $crate::machine::StateID(stringify!($s_out).to_string()),
                }
            );
        )*
        _transtion_table
    }};

}

pub use crate::{alph, input, states, transitions};

#[cfg(test)]
mod tests {
    use crate::machine::{StateID, Symbol};

    #[test]
    fn transitions_macro_builds_a_state_transition_map() {
        let transitions = transitions![
            (s0, '0') -> (s0, '1', >),
            (s0, _) -> (s1, _, >),
        ];

        let state = transitions.get(&StateID("s0".to_string())).unwrap();
        assert_eq!(
            state[&Symbol::AlphabetSymbol('0')].state,
            StateID("s0".to_string())
        );
        assert_eq!(state[&Symbol::Blank].state, StateID("s1".to_string()));
    }
}
