use rand::Rng;
pub mod test_mod{
    pub fn test_mod_fn(){
        println!("test_mod_fn");
    }
    pub mod test_submod{
        pub fn test_submod_fn(){
            println!("test_submod_fn");
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    crate::test_mod::test_mod_fn();
    crate::test_mod::test_submod::test_submod_fn();

    test_mod::test_mod_fn();
    test_mod::test_submod::test_submod_fn();

}
