extern crate bincode;
use state::{State};
use std::fs::File;
use bincode::SizeLimit;

pub fn load_maybe(infile: Option<String>, num: usize) -> State {
    match infile {
        Some(fname) => load_state(fname),
        _ => new_state(num),
    }
}

pub fn write_out(state: &State, outfile: String) {
    let mut out = File::create(outfile).unwrap();
    bincode::encode_into(&state, &mut out, SizeLimit::Infinite).unwrap();
}

pub fn load_state(fname: String) -> State {
    let mut file = File::open(fname).unwrap();
    bincode::decode_from(&mut file, SizeLimit::Infinite).unwrap()
}

pub fn new_state(num: usize) -> State {
    let mut state = State::init();
    state.start(num);
    state
}

