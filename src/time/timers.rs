use std::thread::sleep;
use std::time::Duration;

use crate::telemetry::Logger;
use rand::prelude::ThreadRng;
use rand::Rng;

// Initial wait
const INITIAL_WAIT_SECONDS: u64 = 30;
// Min seconds between actions
const MIN_SECONDS: u64 = 180;
// Max seconds between actions
const MAX_SECONDS: u64 = 480;

/// Wait random seconds
pub fn random_wait(rng: &mut ThreadRng, logger: &Logger) {
    sleep(Duration::from_secs(random_seconds(rng, logger)));
}

/// Initial wait in seconds
pub fn initial_wait(logger: &Logger) {
    logger.info(&format!(
        "ESPERA INICIAL DE {INITIAL_WAIT_SECONDS} SEGUNDOS..."
    ));
    logger.separator();

    sleep(Duration::from_secs(INITIAL_WAIT_SECONDS));
}

/// Generate random seconds in a range
fn random_seconds(rng: &mut ThreadRng, logger: &Logger) -> u64 {
    let random_time = rng.gen_range(MIN_SECONDS..=MAX_SECONDS);

    logger.info(&format!(
        "ESPERANDO PARA LA PROXIMA ACCION {random_time} SEGUNDOS..."
    ));
    logger.separator();

    random_time
}
