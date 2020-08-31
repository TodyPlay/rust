use std::sync::mpsc;
use std::thread;

fn main() {
    single_message();
}

fn single_message() {
    let (tx, rx) = mpsc::channel();

    let _th = thread::spawn(move || {
        let message = vec!["hello".to_string(), "world".to_string(), "none".to_string()];

        for me in message {
            tx.send(me).unwrap();
        }
    });

    for message in rx {
        println!("{}", message);
    }
}