use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

pub trait Observer<E> {
    fn on_notify(&self, event: &E);
}

pub trait MutObserver<E> {
    fn on_notify(&mut self, event: &E);
}

impl<E> Observer<E> for Rc<dyn Observer<E>> {
    fn on_notify(&self, event: &E) {
        (**self).on_notify(event)
    }
}

impl<E> Observer<E> for Rc<RefCell<dyn Observer<E>>> {
    fn on_notify(&self, event: &E) {
        self.borrow().on_notify(event)
    }
}

impl<E> Observer<E> for Rc<RefCell<dyn MutObserver<E>>> {
    fn on_notify(&self, event: &E) {
        self.borrow_mut().on_notify(event)
    }
}

impl<E> Observer<E> for Arc<dyn Observer<E>> {
    fn on_notify(&self, event: &E) {
        (**self).on_notify(event)
    }
}

impl<E> Observer<E> for Arc<Mutex<dyn Observer<E>>> {
    fn on_notify(&self, event: &E) {
        self.lock().unwrap().on_notify(event)
    }
}

impl<E> Observer<E> for Arc<Mutex<dyn MutObserver<E>>> {
    fn on_notify(&self, event: &E) {
        self.lock().unwrap().on_notify(event)
    }
}

impl<E> Observer<E> for Arc<RwLock<dyn Observer<E>>> {
    fn on_notify(&self, event: &E) {
        self.read().unwrap().on_notify(event)
    }
}

impl<E> Observer<E> for Arc<RwLock<dyn MutObserver<E>>> {
    fn on_notify(&self, event: &E) {
        self.write().unwrap().on_notify(event)
    }
}
