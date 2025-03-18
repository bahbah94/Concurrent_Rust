use std::thread;
use channels::Channel;

fn main() {
    // Create a new channel
    let mut channel = Channel::new();
    
    // Use thread::scope to ensure all threads complete before the channel is dropped
    thread::scope(|s| {
        // Split the channel into sender and receiver
        let (sender, receiver) = channel.split();
        
        // Spawn a thread that will send a message
        s.spawn(move || {
            println!("Sender thread: sending message");
            
            // Simulate some work
            thread::sleep(std::time::Duration::from_millis(100));
            
            // Send the message
            sender.send("Hello from another thread!");
            
            println!("Sender thread: message sent");
        });
        
        // In the main thread, receive the message
        println!("Main thread: waiting for message");
        let message = receiver.receive();
        println!("Main thread: received message: {}", message);
        
        assert_eq!(message, "Hello from another thread!");
    });
    
    println!("Communication completed successfully!");
}