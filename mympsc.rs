//Rust's official docs actually recommend preferring channels over Mutex where possible.
//Mutex is like a shared whiteboard 📋
// Everyone walks up to the same board
// Only one person can write at a time
// Others wait and watch

// Channel is like an email/mailbox 📬
// You write a message and send it
// The receiver reads it on their own time
// Sender and receiver never need to meet
use std::sync::mpsc;
use std::time::Duration;

pub fn test_channels(){
    let (transmitter, receiver) = mpsc::channel::<u8>();
    
    receiver.recv_timeout(Duration::from_millis(300));
    
    let processor_code = move || {
        println!("Starting processor thread...");
        let mut failed_count = 0u8;
        loop {
            println!("Ateemptiong to recieve message from channel");
            let receiver_result = receiver.recv_timeout(Duration::from_millis(800));
            if receiver_result.is_ok() {
                println!("Received message: {}", receiver_result.unwrap());
            }
            else {
                failed_count+=1
            };
            if failed_count > 10 {
                println!("Abording processor thread... No work available");
                break;
            }
        }
    };

    for x in 1..=6 {
        let send_result = transmitter.send(x);
        println!("Send status: {}", send_result.is_ok());
        std::thread::sleep(Duration::from_millis(300));
    };

    std::thread::spawn(processor_code).join();
}