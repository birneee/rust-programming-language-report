extern crate rand;
use rand::Rng;
use std::{thread, time};
use std::sync::{Arc, Condvar, Mutex};

const PARKING_SPACES: u32 = 3;
const CARS: u32 = 5;

fn main(){
    let mut join_handles = Vec::with_capacity(CARS as usize);
    let mutex = Mutex::new(PARKING_SPACES);
    let cond = Condvar::new();
    let mutcond = Arc::new((mutex, cond));
    
    for i in 0..CARS{
        let mutcond = mutcond.clone();
        let handle = thread::spawn(move ||{
            let car = Car::new(mutcond, i);
            car.park();
            car.shop();
            car.unpark();
        });
        join_handles.push(handle);
    }

    for handle in join_handles{
        handle.join().expect("Thread join panicked");
    }
}

struct Car{
    mutcond: Arc<(Mutex<u32>, Condvar)>,
    number: u32
}

impl Car{
    fn new(mutcond: Arc<(Mutex<u32>, Condvar)>, number: u32) -> Car {
        Car{
            mutcond: mutcond,
            number: number
        }
    }

    fn park(&self){
        let (ref mutex, ref cond) = *self.mutcond;
        let mut free_spaces = mutex.lock().unwrap();
        println!("CAR {}: searching for parking space", self.number);
        println!("CAR {}: found {} free parking spaces", self.number, free_spaces);
        while *free_spaces == 0 {
            println!("CAR {}: waiting for free space", self.number);
            free_spaces = cond.wait(free_spaces).unwrap();
            println!("CAR {}: waking up", self.number);
            println!("CAR {}: searching for parking space", self.number);
            println!("CAR {}: found {} free parking spaces", self.number, free_spaces);
        }
        *free_spaces -= 1;
        println!("CAR {}: parking", self.number);
    }

    fn shop(&self){
        let mut rng = rand::thread_rng();
        let time = rng.gen_range(100, 1000);
        println!("CAR {}: go shopping for {} ms", self.number, time);
        thread::sleep_ms(time);
    }

    fn unpark(&self){
        let (ref mutex, ref cond) = *self.mutcond;
        let mut free_spaces = mutex.lock().unwrap();
        *free_spaces += 1;
        println!("CAR {}: unparking", self.number);
        cond.notify_one();
    }
}