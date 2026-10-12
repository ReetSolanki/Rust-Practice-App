use std::thread::spawn;
pub fn test_threads(){
    let mut x: u128 = 0u128;
    for i in 1..500 {
        x+=i;
    }
    println!("\x1b[38;2;100;100;255mMain thread finished a little bit of work\x1b[0m");
}

pub fn spawn_thread(){
    let thread_fn = || {
        let mut x: u128 = 0u128;
        for i in 1..50_000_000 {
            x+=i;
        }
        println!("The Value of x is: {x}");
    };
    println!("Starting new worker thread");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Worker thread completed");

    // test_threads();

    //We use join function to tell the main thread to block any other code until the handle thread(worker thread) is completely executed
    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished(){
            println!("All the workers are done!");
            break;
        }
    }
}