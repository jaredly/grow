use std::mem::transmute;
use std::sync::mpsc;
use std::thread;
use std::slice;
use std::sync::mpsc::{Sender, Receiver};
use std::mem;

/* I don't know how to get lifetimes working here... :/

pub fn parallel_full<Data: 'static + Sync, Msg1: Send, Msg2: Send> (data: &Data, parallelism: usize, work: fn(usize, &Data, (&Sender<Msg1>, &Sender<Msg2>)), finish: fn((&Receiver<Msg1>, &Receiver<Msg2>))) {

    let (sender, receiver) = mpsc::channel();
    let (close_sender, close_receiver) = mpsc::channel();
    // TODO make data's lifetime work... as in, be the same as the 
    // let data = unsafe {mem::transmute(data)};
    let data: &Data = unsafe {mem::transmute(data)};
    for i in 0..parallelism {
        let sender = sender.clone();
        let close_sender = close_sender.clone();
        thread::spawn(move || {
            //let data: &Data = unsafe {mem::transmute(data)};
            work(i, data, (&sender, &close_sender));
        });
    }

    // if we don't do this, everything will hang :)
    drop(sender);
    drop(close_sender);
    finish((&receiver, &close_receiver));

}
*/

/// Run a function in parallel over chunks of an array.
pub fn parallel<'data, Data: Send> (data: &'data mut [Data], parallelism: usize, work: fn(usize, usize, &mut [Data])) {
    let chunk_size = (data.len() + parallelism - 1) / parallelism;
    assert!(chunk_size*parallelism >= data.len());

    let mut workeridx = 0usize;
    let (tx, rx) = mpsc::channel();
    for (i, chunk) in data.chunks_mut(chunk_size).enumerate() {
        workeridx += 1;
        let this_tx = tx.clone();
        // We are splitting up the input data into chunks, and sending them to worker tasks.  This
        // requires unsafe code on both the parent and child tasks behalf, since we need to avoid
        // the wrath of the type system.  This is safe since we ensure the parent blocks until all
        // children have completed their processing.
        // let chunk = unsafe { transmute(chunk) };
        let raw_ptr: &usize = unsafe { transmute(chunk.as_mut_ptr()) };
        let len = chunk.len();
        thread::spawn(move || {
            let chunk = unsafe {slice::from_raw_parts_mut(transmute(raw_ptr), len)};
            work(i, i * chunk_size, chunk);
            this_tx.send(()).unwrap();
        });
    }
    for _ in 0 .. workeridx {
        rx.recv().unwrap(); // we receive one message from each job on its completion
    }
}
