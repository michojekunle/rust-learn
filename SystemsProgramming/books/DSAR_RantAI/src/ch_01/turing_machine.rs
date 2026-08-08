use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum State {
    Start,
    Halt,
    AddingNoCarry,
    AddingWithCarry,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MachineError {
    MissingTransition,
}

pub struct TuringMachine {
    tape: Vec<char>,
    offset: usize,
    head: isize,
    state: State,
    transitions: HashMap<(State, char), Transition>,
}

struct Transition {
    write: char,
    direction: Direction,
    next_state: State,
}

const BLANK: char = '_';

impl TuringMachine {
    fn new(input: &str) -> Self {
        let mut machine = Self {
            tape: match input {
                "" => vec![BLANK],
                _ => input.chars().collect(),
            },
            offset: 0,
            head: 0,
            state: State::Start,
            transitions: HashMap::new(),
        };

        machine.build_basic_machine();

        machine
    }

    fn run(&mut self) -> Result<(), MachineError> {
        while self.state != State::Halt {
            let index = self.tape_index();

            let symbol = self.tape[index];

            // what to be done with the symbol
            let transition = self
                .transitions
                .get(&(self.state, symbol))
                .ok_or(MachineError::MissingTransition)?;

            // write
            self.tape[index] = transition.write;

            // change state
            self.state = transition.next_state;

            // move
            self.move_head(transition.direction);
        }
        Ok(())
    }

    fn result(&mut self) -> String {
        todo!()
    }

    fn tape_as_string(&self) -> String {
        self.tape.iter().collect()
    }

    fn tape_index(&self) -> usize {
        (self.head + self.offset as isize) as usize
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.head += 1,
            Direction::Left => self.head -= 1,
        }

        if self.head < 0 {
            self.tape.insert(0, BLANK);
            self.offset += 1;
        } else if self.head as usize >= self.tape.len() {
            self.tape.push(BLANK);
        }
    }

    fn add_transition(
        &mut self,
        state: State,
        read: char,
        write: char,
        direction: Direction,
        next_state: State,
    ) {
        self.transitions.insert(
            (state, read),
            Transition {
                write,
                direction,
                next_state,
            },
        );
    }

    fn build_basic_machine(&mut self) {
        self.add_transition(State::Start, '1', 'X', Direction::Right, State::Start);

        self.add_transition(State::Start, '0', '0', Direction::Right, State::Start);

        self.add_transition(State::Start, BLANK, BLANK, Direction::Left, State::Halt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================
    // INITIALIZATION
    // =========================

    #[test]
    fn initializes_machine_correctly() {
        let machine = TuringMachine::new("101");

        assert_eq!(machine.state, State::Start);
        assert_eq!(machine.head, 0);
        assert_eq!(machine.offset, 0);
        assert_eq!(machine.tape_as_string(), "101");
    }

    #[test]
    fn initializes_empty_input_with_blank() {
        let machine = TuringMachine::new("");

        assert_eq!(machine.tape_as_string(), "_");
        assert_eq!(machine.head, 0);
        assert_eq!(machine.offset, 0);
    }

    // =========================
    // HEAD MOVEMENT
    // =========================

    #[test]
    fn moving_right_grows_tape() {
        let mut machine = TuringMachine::new("101");

        machine.head = 2;
        machine.move_head(Direction::Right);

        assert_eq!(machine.head, 3);
        assert_eq!(machine.tape_as_string(), "101_");
    }

    #[test]
    fn moving_left_grows_tape() {
        let mut machine = TuringMachine::new("101");

        machine.move_head(Direction::Left);

        assert_eq!(machine.head, -1);
        assert_eq!(machine.offset, 1);
        assert_eq!(machine.tape_as_string(), "_101");
    }

    // =========================
    // BASIC TURING MACHINE
    // =========================

    #[test]
    fn replaces_all_ones() {
        let mut machine = TuringMachine::new("11101");

        machine.run().unwrap();

        assert_eq!(machine.tape_as_string(), "XXX0X_");
    }

    #[test]
    fn leaves_zeroes_unchanged() {
        let mut machine = TuringMachine::new("00000");

        machine.run().unwrap();

        assert_eq!(machine.tape_as_string(), "00000_");
    }

    #[test]
    fn handles_mixed_input() {
        let mut machine = TuringMachine::new("101101001");

        machine.run().unwrap();

        assert_eq!(machine.tape_as_string(), "X0XX0X00X_");
    }

    #[test]
    fn handles_single_one() {
        let mut machine = TuringMachine::new("1");

        machine.run().unwrap();

        assert_eq!(machine.tape_as_string(), "X_");
    }

    #[test]
    fn handles_single_zero() {
        let mut machine = TuringMachine::new("0");

        machine.run().unwrap();

        assert_eq!(machine.tape_as_string(), "0_");
    }

    // =========================
    // TERMINATION
    // =========================

    #[test]
    fn machine_halts_after_processing_input() {
        let mut machine = TuringMachine::new("10101");

        machine.run().unwrap();

        assert_eq!(machine.state, State::Halt);
    }

    // =========================
    // ERROR HANDLING
    // =========================

    #[test]
    fn returns_error_when_transition_is_missing() {
        let mut machine = TuringMachine::new("101");

        // Remove the transition that handles '1'.
        machine.transitions.remove(&(State::Start, '1'));

        let result = machine.run();

        assert!(matches!(result, Err(MachineError::MissingTransition)));
    }
}
