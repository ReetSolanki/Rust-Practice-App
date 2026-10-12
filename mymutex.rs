//Mutex = Mutual Exclusion. It's a synchronization primitive that ensures only one thread can access shared data at a time. Only one thread touches the data at a time.
use std::{ops::AddAssign, sync::Mutex};
use std::thread::{scope, sleep};
use std::time::Duration;
pub fn test_mutex(){
    let score = Mutex::new(0u16);

    // let unlocked_data = score.lock(); //score += 5; This cannot be done
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);
    
    // println!("{:?}", data);
    // drop(data);

    let myfunc = || {
        println!("Thread 1 is waiting for mutex lock ...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            sleep(Duration::from_millis(400));
        }
    };
    
    let myfunc2 = || {
        loop {
            println!("Thread 2 is waiting for mutex lock ...");
            let guard = score.try_lock();
            
            if guard.is_ok(){
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 2 is adding {i}");
                };
                break;
            }
            sleep(Duration::from_millis(300));
        }
    };

    _ = scope(|s| { // Join is used to synchronize our child thread with the main thread
        s.spawn(myfunc);
        s.spawn(myfunc2);
        // if handle2.is_err() {
        //     println!("Thread 2 had an error");
        // }
    }); 

    println!("{:?}", score.lock().unwrap());
}   