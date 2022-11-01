use crate::animal::error::AnimalError;

use super::Result;

#[derive(Debug)]
pub(crate) struct Dog {
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

    pub fn take_bone(&mut self) -> Option<Bone> {
        println!("You attempt to take the bone!");
        let result = self.bone.take();
        if result.is_some() {
            println!("You grab the bone! The dog stares at you, bewildered.");
        } else {
            println!("There was no bone to grab silly!");
        }
        result
    }

    pub fn receive_bone(&mut self, bone: Bone) -> Result<()> {
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
pub(crate) struct Bone {
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
pub(crate) enum BoneKind {
    BaconFlavored,
    TurkeyAndStuffing,
    PeanutButter,
}
