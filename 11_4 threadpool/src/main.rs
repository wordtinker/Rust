use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use failure::Error;
use exitfailure::ExitFailure;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;
use threadpool::ThreadPool;

enum Message {
    Started,
    Finished,
    Interrupted
}

fn do_job(running: Arc<AtomicBool>) -> Result<(), Error> {
    // Channel for communication
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(4);

    let batch_size = 100;
    for _ in 1..=batch_size {
        let tx = tx.clone();
        let r = running.clone();
        pool.execute(move || {
            if !r.load(Ordering::SeqCst) {
                tx.send(Message::Interrupted).unwrap();
                return;
            }
            tx.send(Message::Started).unwrap();
            thread::sleep(Duration::from_secs(1));
            tx.send(Message::Finished).unwrap();
        });
    }

    let mut ready = 0;
    // This will block main until iterator yields None
	// which will never happen in case of threadpool
    for received in rx {
        // handle messages
        match received {
            Message::Started => {
                println!("Thread started");
            },
            Message::Finished => {
                println!("Thread finished");
                ready += 1;
            },
            Message::Interrupted => {
                ready += 1;
            }
        }
        if ready == batch_size { break; } // every thread died somehow
    }
    pool.join();
    println!("Exit function");
    Ok(())
}

/// Stop executions with ctrl+c
/// Graceful exit from threadpool
fn main() -> Result<(), ExitFailure> {
    // // Cancellation token
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    // Bind cancellation token to ctrl+c command
    ctrlc::set_handler(move || {
         r.store(false, Ordering::SeqCst);
    })?;
    do_job(running)?;
    Ok(())
}
