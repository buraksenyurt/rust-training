use basic::*;
use std::thread;

mod basic;

fn main() {
    // Case #1
    // start_a_simple_thread();
    // println!("After the thread calling");

    // Case #2
    // move_keyword_error();
    // move_keyword_success();
    multiple_threads_sample();

    println!("After the thread calling");
}
