use crate::call::call_me;

pub fn say_good_morning(name: &str) {
    println!("Good morning, {}", name);
    call_me()
}