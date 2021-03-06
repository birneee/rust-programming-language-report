fn main(){
    trait Hash {
        fn hash(&self) -> u64;
    }

    impl Hash for bool {
        fn hash(&self) -> u64 {
            if *self { 0 } else { 1 }
        }
    }

    impl Hash for i64 {
        fn hash(&self) -> u64 {
            *self as u64
        }
    }

    println!("{}", 8.hash());
    println!("{}", (-8).hash());
    println!("{}", true.hash());
    println!("{}", false.hash());
}