use std::sync::{Arc, Mutex};

pub fn log_queue_len(
    logger: &Arc<Mutex<Vec<String>>>,
    queue:  &Arc<Mutex<Vec<u32>>>,
) {
    // 1️⃣ First lock LOGGER and hold it
    let mut logger_guard = logger.lock().unwrap();

    // 2️⃣ Then lock QUEUE while still holding LOGGER lock
    let len = queue.lock().unwrap().len();
    logger_guard.push(format!("Queue length now {len}"));
}
