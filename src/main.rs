mod worker;
mod logger;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Shared state ----------------------------------------------------------
    let queue  : Arc<Mutex<Vec<u32>>>     = Arc::new(Mutex::new(Vec::new()));
    let logger : Arc<Mutex<Vec<String>>>  = Arc::new(Mutex::new(Vec::new()));

    // -----------------------------------------------------------------------
    // Producer thread: pushes jobs, then processes one job immediately
    // This thread locks resources in order: QUEUE -> LOGGER
    // -----------------------------------------------------------------------
    let q_for_producer   = Arc::clone(&queue);
    let log_for_producer = Arc::clone(&logger);
    let producer_handle = thread::spawn(move || {
        for job_id in 0..500 {
            q_for_producer.lock().unwrap().push(job_id);
            // *Intentional* longer yield to increase chance of deadlock
            thread::sleep(Duration::from_millis(1));
            worker::process_one(&q_for_producer, &log_for_producer);
        }
    });

    // -----------------------------------------------------------------------
    // Logger thread: emits queue length frequently
    // This thread locks resources in order: LOGGER -> QUEUE
    // -----------------------------------------------------------------------
    let q_for_logger   = Arc::clone(&queue);
    let log_for_logger = Arc::clone(&logger);
    let logger_handle = thread::spawn(move || {
        loop {
            logger::log_queue_len(&log_for_logger, &q_for_logger);
            // Shorter sleep to increase chance of deadlock
            thread::sleep(Duration::from_micros(500));

            // Exit once the queue is empty and producer finished
            if q_for_logger.lock().unwrap().is_empty() {
                break;
            }
        }
    });

    // Wait for both threads --------------------------------------------------
    producer_handle.join().unwrap();
    logger_handle.join().unwrap();

    println!("Done!  Logged {} lines.",
             logger.lock().unwrap().len());
}
