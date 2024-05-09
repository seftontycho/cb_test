use std::{
    ffi::{c_int, c_void},
    ops::Mul,
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc, Mutex,
    },
};

use rayon::prelude::*;

pub struct Task<In, Out, State> {
    task: fn(In, Arc<Mutex<State>>) -> Out,
    data: In,
    cb_data: *mut c_void,
    cb: extern "C" fn(*mut c_void, Out) -> (),
}

pub struct Sender {
    tx: SyncSender<Task<i32, i32, i32>>,
    rx: Receiver<()>, // One shot channel to signal the all tasks are done
}

unsafe impl<In, Out, State> Send for Task<In, Out, State> {}

fn process_calls<In: 'static, Out: 'static, State: Sync + Send + 'static>(
    rx_task: Receiver<Task<In, Out, State>>,
    tx_done: SyncSender<()>,
    state: State,
    channel_size: usize,
) {
    let state = Arc::new(Mutex::new(state));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(channel_size)
        .build()
        .unwrap();

    pool.scope(|_| {
        rx_task.into_iter().par_bridge().for_each(|task| {
            let state = state.clone();

            execute_task(task, state);
        });
    });

    tx_done.send(()).unwrap();
}

fn execute_task<In, Out, State>(task: Task<In, Out, State>, state: Arc<Mutex<State>>) {
    let task_result = (task.task)(task.data, state);

    (task.cb)(task.cb_data, task_result);
}

/// Will block until all processing has finished.
/// Drops the sender, trying to use the state after this is a use-after-free.
#[no_mangle]
pub unsafe extern "C" fn flush(sender: *mut Sender) {
    let Sender { rx, tx } = *Box::from_raw(sender);

    drop(tx);

    rx.recv().unwrap();
}

// This is how c++ will create the state.
#[no_mangle]
pub extern "C" fn init(channel_size: c_int) -> *mut Sender {
    let (tx_task, rx_task) = std::sync::mpsc::sync_channel(channel_size as usize);
    let (tx_done, rx_done) = std::sync::mpsc::sync_channel(1);

    let state = 100;

    std::thread::spawn(move || {
        process_calls(rx_task, tx_done, state, channel_size as usize);
    });

    Box::into_raw(Box::new(Sender {
        tx: tx_task,
        rx: rx_done,
    }))
}

/// This is how c++ will queue tasks.
/// Will block if queue/buffer is full.
/// `data` can be assumed to have been copied by the time this fn returns.
#[no_mangle]
pub unsafe extern "C" fn run(
    sender: *mut Sender,
    cb_data: *mut c_void,
    cb: extern "C" fn(*mut c_void, c_int),
    data: c_int,
) {
    let sender = Box::from_raw(sender);

    sender
        .tx
        .send(Task {
            task: multiply_by,
            data: data.clone(),
            cb_data,
            cb,
        })
        .unwrap();

    Box::into_raw(sender);
}

/// An example task
fn multiply_by<State>(data: i32, state: Arc<Mutex<State>>) -> i32
where
    State: Mul<i32, Output = i32> + Copy,
{
    // Simulates preprocessing time
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let state = state.lock().unwrap();
    let output = *state * data;

    // Simulates time to run actual model
    std::thread::sleep(std::time::Duration::from_millis(1));

    drop(state);

    

    output
}
