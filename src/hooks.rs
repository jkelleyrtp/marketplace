use std::{cell::RefCell, collections::HashSet, rc::Rc};

use dioxus::{core::ScopeId, prelude::*};

use crate::state::GlobalModel;

// Tracks all the subscribers to a shared State
pub(crate) struct ProvidedStateInner<T> {
    value: Rc<RefCell<T>>,
    notify_any: Rc<dyn Fn(ScopeId)>,
    consumers: HashSet<ScopeId>,
}

impl<T> ProvidedStateInner<T> {
    pub(crate) fn notify_consumers(&mut self) {
        for consumer in self.consumers.iter() {
            (self.notify_any)(*consumer);
        }
    }
}
