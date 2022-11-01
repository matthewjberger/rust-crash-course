use crate::{
    animal::{
        error::AnimalError,
        traits::{Birthday, Speak},
    },
    Result,
};
use rand::Rng;

pub(crate) struct Cat {
    age: u8,
}

impl Default for Cat {
    fn default() -> Self {
        Self { age: 1 }
    }
}

impl Speak for Cat {
    fn speak(&self) -> Result<()> {
        let number = rand::thread_rng().gen_range(0..=1);
        if number & 1 == 0 {
            Err(Box::new(AnimalError::new(
                "The cat did not want to speak. Instead, it stares at you silently.",
            )))
        } else {
            println!("meow...");
            Ok(())
        }
    }
}

impl Birthday for Cat {
    fn celebrate_birthday(&mut self) {
        self.age = self.age + 1;
        println!("Snooty face is {} furrowed brows old!", self.age);
    }
}
