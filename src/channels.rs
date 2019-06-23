fn main(){
    extern crate rand;
    use rand::Rng;
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    for i in 0..10{
        let tx_thread = tx.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let duration = rng.gen::<u32>() % 1000;
            thread::sleep_ms(duration);
            tx_thread.send(i).unwrap();
        });
    }

    drop(tx);

    for message in rx{
        print!("{}", message);
    }
}