use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::{ThreadPool, ThreadPoolBuilder};

pub static THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    ThreadPoolBuilder::new()
        .num_threads(num_threads())
        .build()
        .expect("Failed to create thread pool")
});
static NUM_THREADS_INIT: AtomicUsize = AtomicUsize::new(0);

fn num_threads() -> usize {
    match NUM_THREADS_INIT.load(Ordering::Relaxed) {
        0 => {
            let num_threads = std::env::var("HERBOLUTION_WORKER_THREADS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_else(num_cpus::get);
            num_threads
        }
        x => x,
    }
}

pub fn set_num_threads(num_threads: usize) {
    let num_cpus = num_cpus::get();
    if num_threads == 0 || num_threads > num_cpus {
        panic!("Number of threads must be greater than 0 and less than or equal to {num_cpus}.");
    }
    NUM_THREADS_INIT.store(num_threads, Ordering::Relaxed);
}
