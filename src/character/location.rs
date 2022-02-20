pub enum Location {
    City,
    Stronghold,
}

impl Default for Location {
    fn default() -> Self {
        Self::City
    }
}

impl Location {
    pub fn change_location(&mut self, new_location: Location) {
        *self = new_location;
    }
}
