#![allow(unused_variables, dead_code)]

struct User {
    name: String,
    weight: f32,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        unimplemented!();
    }

    pub fn name(&self) -> &str {
        unimplemented!();
    }

    pub fn age(&self) -> u32 {
        unimplemented!();
    }

    pub fn weight(&self) -> f32 {
        unimplemented!();
    }

    pub fn set_age(&mut self, age: u32) {
        unimplemented!();
    }

    pub fn set_weight(&mut self, weight: f32) {
        unimplemented!();
    }
}

fn main() {
    println!("Hello, world!");

    let bob = User::new(String::from("Hai Tien"), 26, 61.0);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {}

#[test]
fn test_age() {}
