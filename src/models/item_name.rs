pub trait ItemName {
    fn name(&self) -> String;

    fn name_upper(&self) -> String {
        self.name().to_uppercase()
    }
}
