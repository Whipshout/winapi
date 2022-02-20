use rand::prelude::ThreadRng;
use rand::Rng;

use crate::character::{Mount, Player};
use crate::telemetry::Logger;
use crate::time::random_wait;

pub struct Choice {
    pub player: Player,
    pub logger: Logger,
}

impl Choice {
    pub fn new(player: Player, logger: Logger) -> Self {
        Self { player, logger }
    }

    /// # Safety
    pub unsafe fn random_choice(&mut self, rng: &mut ThreadRng) {
        loop {
            match self.player.mount {
                Mount::Mounted => {
                    match Choice::random_bool(rng) {
                        false => self.player.change_mount_state(Mount::Unmounted),
                        true => self.player.execute(self.player.random_mount_action(rng)),
                    };
                }
                Mount::Unmounted => {
                    match Choice::random_bool(rng) {
                        false => self.player.change_mount_state(Mount::Mounted),
                        true => self.player.change_current_location(),
                    };
                }
            };

            random_wait(rng, &self.logger);
        }
    }

    /// Generate random bool with weight
    pub fn random_bool(rng: &mut ThreadRng) -> bool {
        rng.gen_bool(0.5)
    }
}
