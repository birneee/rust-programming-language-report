fn main(){
    use std::thread;
    use std::sync::{Arc, Mutex};

    let num = Arc::new(Mutex::new(0));

    for _ in 0..10{
        let num = num.clone();
        thread::spawn(move || {
            let mut num = num.lock().unwrap();
            *num += 1;
        });
    }

    thread::sleep_ms(50);

    println!("{}", num.lock().unwrap());
}