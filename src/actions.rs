use penrose::{
    core::{
        bindings::KeyEventHandler,
        helpers::spawn,
    },
};

use crate::{Conn, Wm, INITIAL_SCREEN_TEMP};

use std::sync::atomic::{AtomicI32, Ordering};

// need this static atomic due to the boxed closures used for key event handling
static REDSHIFT_TEMP: AtomicI32 = AtomicI32::new(INITIAL_SCREEN_TEMP);

pub fn inc_redshift(val: i32) -> KeyEventHandler<Conn> {
    Box::new(move |_: &mut Wm| {
        REDSHIFT_TEMP.fetch_add(val, Ordering::Relaxed);
        let temp = REDSHIFT_TEMP.load(Ordering::Relaxed);
        spawn(format!("redshift -P -O {}", temp))
    })
}
