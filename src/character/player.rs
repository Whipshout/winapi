use rand::prelude::{SliceRandom, ThreadRng};

use crate::character::{input, Location, Mount};
use crate::telemetry::Logger;
use crate::windows::WindowHandler;

// Mount action keys
const MOUNT_KEYS: [usize; 3] = [0x51, 0x57, 0x45];

pub struct Player {
    pub location: Location,
    pub mount: Mount,
    pub window_handler: WindowHandler,
    pub logger: Logger,
}

impl Player {
    pub fn new(window_handler: WindowHandler, logger: Logger) -> Self {
        Self {
            location: Default::default(),
            mount: Default::default(),
            window_handler,
            logger,
        }
    }

    /// # Safety
    pub unsafe fn execute(&self, key: usize) {
        input(&self.window_handler, key);
    }

    /// # Safety
    /// Change mount state between mounted and unmounted
    pub unsafe fn change_mount_state(&mut self, new_state: Mount) {
        match new_state {
            Mount::Mounted => self.logger.info("ME MONTO EN EL CABALLO !!!"),
            Mount::Unmounted => self.logger.info("ME BAJO DEL CABALLO !!!"),
        };

        self.execute(0x35);

        self.mount.change_state(new_state);
    }

    /// Choose random mount action
    pub fn random_mount_action(&self, rng: &mut ThreadRng) -> usize {
        self.logger.info("HABILIDAD DE LA MONTURA !!!");

        *MOUNT_KEYS.choose(rng).unwrap()
    }

    /// # Safety
    /// Change location between City and Stronghold
    pub unsafe fn change_current_location(&mut self) {
        match self.location {
            Location::City => {
                self.execute(0x36);
                self.location.change_location(Location::Stronghold);

                self.logger.info("ME VOY AL STRONGHOLD !!!");
            }
            Location::Stronghold => {
                self.execute(0x70);
                self.location.change_location(Location::City);

                self.logger.info("ME VOY A LA CIUDAD !!!");
            }
        };
    }
}
