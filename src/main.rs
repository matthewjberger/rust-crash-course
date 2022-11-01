use animal::traits::{Birthday, Speak};
use cat::Cat;
use dog::{Bone, BoneKind, Dog};

mod animal;
mod cat;
mod dog;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let mut cat = Cat::default();
    cat.celebrate_birthday();
    cat.speak()?;

    let mut dog = Dog::new(8);
    dog.celebrate_birthday();
    dog.speak()?;

    let bone = Bone::from(BoneKind::BaconFlavored);
    dog.receive_bone(bone)?;

    // Uncomment this to give the dog a bone while he already has one!
    //
    // let bone = Bone::from(BoneKind::BaconFlavored);
    // dog.receive_bone(bone)?;

    let bone = dog.take_bone();
    if let Some(bone) = bone.as_ref() {
        println!("That's a high quality {:?} bone buddy!", bone);
    }

    // Uncomment this to look silly by grabbing a bone that you know isn't there
    // let _ = dog.take_bone();

    Ok(())
}
