use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let base_ip = "192.168.1.";
    let num_threads = 255;

    // Create a thread pool
    let pool = ThreadPool::new(num_threads);

    // Create a shared mutable state to collect results
    let results = Arc::new(Mutex::new(vec![]));

    // Iterate through IP addresses and dispatch jobs to the thread pool
    for i in 1..=num_threads {
        let results = Arc::clone(&results);
        let ip = format!("{}.{}", base_ip, i);
        let pool = pool.clone();
        pool.execute(move || {
            let success = ping(&ip);
            let mut results = results.lock().unwrap();
            results.push((ip.parse::<IpAddr>().unwrap(), success));
        });
    }

    // Wait for all threads to finish
    pool.join();

    // Process results
    let results = results.lock().unwrap();
    println!("Results:");
    for (ip, success) in results.iter() {
        println!("{}: {}", ip, if *success { "Reachable" } else { "Unreachable" });
    }
}

// Function to ping an IP address
fn ping(ip: &str) -> bool {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .expect("Failed to execute ping command");
    output.status.success()
}

// Simple thread pool implementation
struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            let thread = thread::spawn(|| {});
            threads.push(thread);
        }
        ThreadPool { threads }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        for thread in &self.threads {
            let f = f.clone();
            thread::spawn(move || {
                f();
            });
        }
    }

    fn join(self) {
        for thread in self.threads {
            thread.join().unwrap();
        }
    }
}

