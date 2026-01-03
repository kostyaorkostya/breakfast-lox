use super::{Env, NativeFn, RuntimeError, Val};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn native_fns(clock: Option<Rc<RefCell<f64>>>) -> Vec<NativeFn> {
    vec![SecSinceUnixEpoch::new(clock)]
}

#[derive(Debug)]
struct SecSinceUnixEpoch;

impl SecSinceUnixEpoch {
    fn new(clock: Option<Rc<RefCell<f64>>>) -> NativeFn {
        type ClockFn = Box<dyn Fn(&mut Env, &[Val; 0]) -> Result<Val, RuntimeError>>;

        let fn_: ClockFn = match clock {
            Some(clock) => Box::new(move |_, _| Ok(Val::Num(*clock.borrow()))),
            None => Box::new(|_, _| {
                Ok(Val::Num(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| RuntimeError::Internal(Box::new(e)))?
                        .as_secs_f64(),
                ))
            }),
        };
        NativeFn::new::<0>("clock".into(), fn_)
    }
}
