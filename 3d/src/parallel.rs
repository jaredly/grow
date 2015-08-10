use std::mem::transmute;
use std::sync::mpsc;
use std::thread;
use std::slice;

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
