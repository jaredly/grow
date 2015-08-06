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
    let mut out = File::create(outfile.clone()).ok().expect(&format!("Can't write to {}", outfile));
    bincode::encode_into(&state, &mut out, SizeLimit::Infinite).ok().expect("Failed to encode state")
}

pub fn load_state(fname: String) -> State {
    let mut file = File::open(fname.clone()).ok().expect(&format!("Can't read from file: {}", fname));
    bincode::decode_from(&mut file, SizeLimit::Infinite).ok().expect(&format!("Unable to load state - is the format right? {}", fname))
}

pub fn new_state(num: usize) -> State {
    let mut state = State::init();
    state.start(num);
    state
}

