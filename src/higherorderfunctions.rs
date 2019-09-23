fn main(){
    fn exec_with_random_values<F>(operator: F)
                     where F : Fn(u32, u32) -> u32{
        let mut rng = rand::thread_rng();
        let a = rng.gen();let b = rng.gen();
        let result = operator(a, b);
        println!("{}⊗{} = {}", a, b, result);
    }

    exec_with_random_values(|a, b| a + b); // closure

    fn add(a: u32, b: u32) -> u32 { a + b }; // function
    exec_with_random_values(add);
}