#![allow(unused_variables, dead_code)]

struct User {
    name: String,
    weight: f32,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }
}

fn main() {
    println!("Hello, world!");

    let bob = User::new(String::from("Hai Tien"), 26, 61.0);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 30, 71.1);
    assert_eq!(bob.weight(), 71.1);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 30, 71.1);
    assert_eq!(bob.age(), 30);

    bob.set_age(29);

    assert_eq!(bob.age(), 29);
}
