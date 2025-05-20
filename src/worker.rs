use std::sync::{Arc, Mutex};

pub fn process_one(
    queue:  &Arc<Mutex<Vec<u32>>>,
    logger: &Arc<Mutex<Vec<String>>>,
) {
    // 1️⃣ First lock QUEUE and hold it
    let mut queue_guard = queue.lock().unwrap();

    if let Some(id) = queue_guard.pop() {
        // 2️⃣ Then lock LOGGER while still holding QUEUE lock
        logger.lock().unwrap()
            .push(format!("Started job {id}"));
        // pretend to work ...
    }
}
