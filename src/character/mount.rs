pub enum Mount {
    Mounted,
    Unmounted,
}

impl Default for Mount {
    fn default() -> Self {
        Self::Unmounted
    }
}

impl Mount {
    pub fn change_state(&mut self, new_state: Mount) {
        *self = new_state;
    }
}
