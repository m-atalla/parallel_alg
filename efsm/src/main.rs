use std::collections::HashMap;


const INIT_STATE_IDX: usize = 0;

#[derive(Debug)]
pub struct State {
    value: char,
    ts: HashMap<u8, usize>,
    quantifiers: Vec<Quantifier>,
}

#[derive(Debug)]
enum Quantifier {
    ExactlyOnce(char),
}

pub enum TsResult {
    NoTransition
}

impl State {
    pub fn nil() -> Self {
        Self {
            value: 0u8 as char,
            ts: HashMap::new(),
            quantifiers: Vec::default()
        }
    }
    pub fn new(c: char) -> Self {
        Self {
            value: c,
            ts: HashMap::new(),
            quantifiers: vec![Quantifier::ExactlyOnce(c)]
        }
    }

    pub fn add_transition(&mut self, new_state_value: char, new_state_idx: usize) {
        self.ts.insert(new_state_value as u8, new_state_idx);
    }

    pub fn next_state(&self, input: char) -> usize {
        match self.ts.get(&(input as u8)) {
            Some(&idx) => idx,
            None => INIT_STATE_IDX
        }
    }
}

#[derive(Debug)]
pub struct Regex {
    pub states: Vec<State>
}

impl Regex {
    pub fn new() -> Self {
        Self { 
            states: vec![State::nil()]
        }
    }

    pub fn push_state(&mut self, new_state: State) {
        let new_state_idx = self.states.len();
        let new_state_value = new_state.value;

        self.states.push(new_state);

        self.states
            .get_mut(new_state_idx - 1)
            .unwrap()
            .add_transition(new_state_value, new_state_idx);
    }


    pub fn parse(&mut self, s: &str) {
        for ch in s.chars() {
            self.push_state(
                State::new(ch)
            );
        }
    }

    pub fn test(&mut self, input: &str) -> bool{
        let mut current_idx = 0;


        for ch in input.chars() {

            current_idx = self.states
                .get(current_idx)
                .unwrap()
                .next_state(ch);

            // reset to initial state on failure
            if current_idx == 0 {
                current_idx = self.states
                    .get(0)
                    .unwrap()
                    .next_state(ch);
            }

            // break on first match, maybe this shouldn't be the case
            // a string may have multiple matches, but this is good enough for now.
            if current_idx == (self.states.len() - 1) {
                break;
            }

        }

        current_idx == (self.states.len() - 1)
    }
}

fn main() {
    let mut regex = Regex::new();

    regex.parse("abi");

    println!("{}", regex.test("aabii"));
}
