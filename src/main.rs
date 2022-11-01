pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug)]
struct AnimalError {
    details: String,
}

impl AnimalError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl std::error::Error for AnimalError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::fmt::Display for AnimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

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

    pub fn speak(&self) -> Result<()> {
        match self.bone.as_ref() {
            Some(bone) => Err(Box::new(AnimalError::new(&format!(
                "Dog can't speak because of the {:?} bone!",
                bone
            )))),
            None => Ok(println!("Woof!")),
        }
    }

    fn receive_bone(&mut self, bone: Bone) -> Result<()> {
        match self.bone.as_ref() {
            Some(bone) => {
                return Err(Box::new(AnimalError::new(&format!(
                    "Dog already has a bone! ({:?})",
                    bone
                ))))
            }
            None => {
                println!("Doggy grabbed the {:?} bone!", bone.kind);
                self.bone = Some(bone);
            }
        };
        Ok(())
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

impl From<BoneKind> for Bone {
    fn from(kind: BoneKind) -> Self {
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
    dog.speak()?;

    let bone = Bone::from(BoneKind::BaconFlavored);
    dog.receive_bone(bone)?;

    // Uncomment this to give the dog a bone while he already has one!
    //
    // let bone = Bone::from(BoneKind::BaconFlavored);
    // dog.receive_bone(bone)?;

    Ok(())
}
