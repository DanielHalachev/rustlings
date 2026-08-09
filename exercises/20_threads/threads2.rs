// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    // NOTE: we have multiple writers, and a reader (main) at the end
    // only one thread should be able to access the value at a time
    // arc ensures multiple thread-safe reference counts, but does not synchronize the access
    // so we guard the access with a mutex, to prevent race conditions when writing
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_done += 1;
            // `job_status` is a MutexGuard, the equivalent of a C++ std::lock_guard.
            // The lambda ends, the guard is dropped, and dropping it unlocks the mutex.
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
