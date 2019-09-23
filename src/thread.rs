fn main(){
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        println!("hi from spawned thread!")
    });

    println!("hi from main thread!");

    handle.join().unwrap();
}