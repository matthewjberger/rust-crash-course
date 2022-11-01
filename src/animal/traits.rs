use crate::Result;

pub(crate) trait Speak {
    fn speak(&self) -> Result<()>;
}

pub(crate) trait Birthday {
    fn celebrate_birthday(&mut self);
}

pub(crate) trait Animal: Speak + Birthday {}
