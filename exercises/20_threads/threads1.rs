// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

// std::thread::spawn = std::thread t (lambda) or std::async(std::launch::async, lambda) in C++.
// when an std::thread in C++ is joined, void () is returned
// for an actual result in C++, you need std::async + std::future
// In Rust, std::thread::spawn returns a handler that always returns a result
// In Rust, you call join() on the handler, in C++ you call t.join() on the thread object itself or fut.get() on the future
// In C++, you capture by value [i]{...}, in Rust the lambda takes ownership of the inputs, instead of borrowing them (move)
// this is because the thread may outlive main's stack frame

// std::vector<std::future<long long>> handles;
// for (int i = 0; i < 10; ++i) {
//     handles.push_back(std::async(std::launch::async, [i] {
//         auto start = std::chrono::steady_clock::now();
//         std::this_thread::sleep_for(std::chrono::milliseconds(250));
//         std::println("Thread {} done", i);
//         return std::chrono::duration_cast<std::chrono::milliseconds>(
//                    std::chrono::steady_clock::now() - start).count();
//     }));
// }

// std::vector<long long> results;
// for (auto& handle : handles) results.push_back(handle.get());

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        let result = handle.join();
        results.push(result.unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
