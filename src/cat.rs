pub(crate) struct Cat {
    age: u8,
}

impl Default for Cat {
    fn default() -> Self {
        Self { age: 1 }
    }
}
