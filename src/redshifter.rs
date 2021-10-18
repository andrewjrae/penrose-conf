use penrose::{
    core::helpers::spawn,
    Result,
};

use crate::INITIAL_SCREEN_TEMP;

use std::sync::atomic::{AtomicI32, Ordering};

// need this static atomic due to the boxed closures used for key event handling
static REDSHIFT_TEMP: AtomicI32 = AtomicI32::new(INITIAL_SCREEN_TEMP);

pub fn apply() -> Result<()> {
    let temp = REDSHIFT_TEMP.load(Ordering::Relaxed);
    spawn(format!("redshift -P -O {}", temp))
}

pub fn inc(val: i32) -> Result<()> {
    REDSHIFT_TEMP.fetch_add(val, Ordering::Relaxed);
    apply()
}

pub fn reset() -> Result<()> {
    REDSHIFT_TEMP.store(INITIAL_SCREEN_TEMP, Ordering::Relaxed);
    apply()
}
