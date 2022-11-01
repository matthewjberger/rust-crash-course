pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug)]
struct Dog {
    age: u8,
    bone: Option<Bone>, // The dog may or may not be holding a Bone
}

impl Default for Dog {
    fn default() -> Self {
        Self { age: 1, bone: None }
    }
}

impl Dog {
    pub fn new(age: u8) -> Self {
        Self {
            age,
            ..Default::default()
        }
    }

    pub fn celebrate_birthday(&mut self) {
        self.age = self.age + 1;
        println!("Wiggly butt is {} wags old!", self.age);
    }

    pub fn speak(&self) {
        println!("Woof!");
    }
}

#[derive(Debug)]
struct Bone {
    kind: BoneKind,
}

impl Bone {
    pub fn new(kind: BoneKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
enum BoneKind {
    BaconFlavored,
    TurkeyAndStuffing,
    PeanutButter,
}

fn main() -> Result<()> {
    let mut dog = Dog::new(8);
    dog.celebrate_birthday();
    dog.speak();
    Ok(())
}
