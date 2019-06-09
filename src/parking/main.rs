extern crate rand;
use rand::Rng;
use std::{thread, time};
use std::sync::{Condvar, Mutex};
use lazy_static::lazy_static;
use time::Duration;

const PARKING_SPOTS: u32 = 100;
const CARS: u32 = 1000;
const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    static ref MUTEX: Mutex<u32> = Mutex::new(PARKING_SPOTS);
    static ref COND: Condvar = Condvar::new();
}

fn main(){
    let mut threads = vec![];

    for _ in 0..CARS{
        threads.push(thread::spawn(||{
            let car = Car::new();
            car.park();
            thread::sleep(random_duration(100, 1000));
            car.unpark();
        }));
    }

    for thread in threads{
        thread.join().expect("The thread being joined has panicked");
    }
}

struct Car{
    license_plate: String
}

impl Car{
    fn new() -> Car {
        Car{
            license_plate: random_license_plate()
        }
    }

    fn park(&self){
        println!("{}: searching for parking spot", self.license_plate);
        {
            let mut free_spots = MUTEX.lock().unwrap();
            while *free_spots == 0 {
                println!("{}: waiting for free spot", self.license_plate);
                free_spots = COND.wait(free_spots).unwrap();
                println!("{}: waking up ({} free spots)", self.license_plate, free_spots);
            }
            *free_spots -= 1;
            println!("{}: parking ({} free spots)", self.license_plate, free_spots);
        }
    }

    fn unpark(&self){
        let mut free_spots = MUTEX.lock().unwrap();
        *free_spots += 1;
        println!("{}: unparking ({} free spots)", self.license_plate, free_spots);
        COND.notify_one();
    }
}

fn random_license_plate() -> String{
    let mut lp = "LA-".to_string();
    for _ in 0..2 {
        lp.push(random_letter());
    }
    lp.push('-');
    lp.push_str(random_number(1000, 9999).to_string().as_str());
    lp
}

fn random_duration(min_ms: u64, max_ms: u64) -> Duration{
    Duration::from_millis(
        random_number(
            min_ms,
            max_ms))
}

fn random_number(min: u64, max: u64) -> u64{
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

fn random_letter() -> char{
    char::from(*LETTERS
        .get(
            random_number(
                0,
                LETTERS.len() as u64
            ) as usize)
        .unwrap())
}