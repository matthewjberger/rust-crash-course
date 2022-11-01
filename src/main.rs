struct Dog {
    age: u8,
}

impl Dog {
    pub fn celebrate_birthday(&mut self) {
        self.age = self.age + 1;
        println!("Wiggly butt is {} wags old!", self.age);
    }
}

fn main() {
    let mut dog = Dog { age: 8 };
    dog.celebrate_birthday();
}
