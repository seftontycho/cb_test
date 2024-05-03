use std::{
    ffi::{c_int, c_void},
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc, Mutex,
    },
};

pub struct Task<In, Out, State> {
    task: fn(In, &State) -> Out,
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
) {
    let state = Arc::new(Mutex::new(state));

    let mut handles = vec![];

    for task in rx_task.iter() {
        let handle = std::thread::spawn({
            let state = state.clone();

            move || {
                execute_task(task, state);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    tx_done.send(()).unwrap();
}

fn execute_task<In, Out, State>(task: Task<In, Out, State>, state: Arc<Mutex<State>>) {
    let state = state.lock().unwrap();

    (task.cb)(task.cb_data, (task.task)(task.data, &state));
}

#[no_mangle]
pub unsafe extern "C" fn flush(sender: *mut Sender) {
    let sender = Box::from_raw(sender);

    let tx = sender.tx;
    let rx = sender.rx;

    drop(tx);

    rx.recv().unwrap();
}

#[no_mangle]
pub extern "C" fn init(channel_size: c_int) -> *mut Sender {
    let (tx_task, rx_task) = std::sync::mpsc::sync_channel(channel_size as usize);
    let (tx_done, rx_done) = std::sync::mpsc::sync_channel(1);

    let state = 100;

    std::thread::spawn(move || {
        process_calls(rx_task, tx_done, state);
    });

    Box::into_raw(Box::new(Sender {
        tx: tx_task,
        rx: rx_done,
    }))
}

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
            data,
            cb_data,
            cb,
        })
        .unwrap();

    Box::into_raw(sender);
}

fn multiply_by(data: i32, state: &i32) -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(1));

    data * state
}
