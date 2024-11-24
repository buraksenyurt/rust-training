use arc_and_mutex::*;
// use basic::*;
// use std::thread;

mod arc_and_mutex;
mod basic;

fn main() {
    // Case #1
    // start_a_simple_thread();
    // println!("After the thread calling");

    // Case #2
    // move_keyword_error();
    // move_keyword_success();
    // multiple_threads_sample();

    // run_with_error();
    run_correctly();
    // run_inconsistent();
    // run_safely();
    // run_mutex();

    println!("After the thread calling");
}
