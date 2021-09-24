use penrose::{
    core::{
        client::Client,
        hooks::Hook,
        xconnection::{XConn, Xid},
    },
    WindowManager, Result
};

use std::collections::HashMap;

pub struct StartupScript {
    path: String,
}

impl StartupScript {
    pub fn new(s: impl Into<String>) -> Box<Self> {
        Box::new( Self { path: s.into() } )
    }
}

impl<X: XConn> Hook<X> for StartupScript {
    fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
        spawn!(&self.path)
    }
}

pub struct CenterFloats {
    default_scale: f64,
    overrides: HashMap<String, f64>,
    ignores: Vec<String>,
}

impl CenterFloats {
    pub fn new(default_scale: f64,
               overrides: HashMap<String, f64>,
               ignores: Vec<String>) -> Box<Self> {
        Box::new(Self {
            default_scale,
            overrides,
            ignores
        })
    }

    fn centered_above<X: XConn>(&self, id: Xid, wm: &mut WindowManager<X>, scale: f64) -> Result<()> {
        if let Some(region) = wm.screen_size(wm.active_screen_index()) {
            let r = region.scale_w(scale).scale_h(scale);
            wm.position_client(id, r.centered_in(&region)?, true)?;
        }
        wm.show_client(id)?;
        Ok(())
    }
}

impl<X: XConn> Hook<X> for CenterFloats {
    fn new_client(&mut self, wm: &mut WindowManager<X>, c: &mut Client) -> Result<()> {
        if let Some(scale) = self.overrides.get(c.wm_class()) {
            self.centered_above(c.id(), wm, *scale)?;
        }
        else if !self.ignores.contains(&c.wm_class().to_string()) {
            self.centered_above(c.id(), wm, self.default_scale)?;
        }

        Ok(())
    }
}
