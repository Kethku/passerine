use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::common::{
    lambda::Lambda,
    data::Data,
};

/// Wraps a `Lambda` with some scope context.
/// > NOTE: currently a work-in-progress.
#[derive(Debug)]
pub struct Closure {
    pub lambda: Lambda,
    pub captured: Vec<Rc<RefCell<Data>>>,
}

impl Closure {
    /// Constructs a new `Closure` by wrapping a `Lambda`.
    pub fn wrap(lambda: Lambda) -> Closure {
        Closure { lambda, captured: vec![] }
    }
}
