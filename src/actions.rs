use penrose::{
    core::{
        bindings::KeyEventHandler,
        helpers::spawn,
    }
};

use crate::{Conn, START_SCRIPT, Wm, redshifter};

pub fn inc_redshift(val: i32) -> KeyEventHandler<Conn> {
    Box::new(move |_: &mut Wm| {
        redshifter::inc(val)
    })
}

pub fn soft_restart() -> KeyEventHandler<Conn> {
    Box::new(move |_: &mut Wm| {
        spawn(START_SCRIPT)?;
        redshifter::apply()
    })
}
