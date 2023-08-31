use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};

fn main() {
    let mut result = String::new();

    for _ in 0..10 {
        let number = thread_rng().gen_range(65..100);
        let ch = from_u32(number).unwrap();
        result.push(ch);
    }
    print!("Password: {}", result)
}
