use crate::observer::{MutObserver, Observer};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

pub struct Subject<E> {
    observers: Vec<Box<dyn Observer<E>>>,
}

impl<E> Default for Subject<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Subject<E> {
    pub fn new() -> Self {
        Self { observers: vec![] }
    }

    pub fn notify(&self, event: &E) {
        for observer in self.observers.iter() {
            observer.on_notify(event)
        }
    }

    pub fn add_observer(&mut self, observer: impl Observer<E> + 'static) -> usize {
        self.observers.push(Box::new(observer));
        self.observers.len() - 1
    }

    pub fn remove_observer(&mut self, index: usize) -> Box<dyn Observer<E>> {
        self.observers.remove(index)
    }
}

impl<E: 'static> Subject<E> {
    pub fn add_rc_observer(&mut self, observer: Rc<dyn Observer<E>>) -> usize {
        self.add_observer(observer)
    }

    pub fn add_rc_refcell_observer(&mut self, observer: Rc<RefCell<dyn Observer<E>>>) -> usize {
        self.add_observer(observer)
    }

    pub fn add_rc_refcell_mut_observer(
        &mut self,
        observer: Rc<RefCell<dyn MutObserver<E>>>,
    ) -> usize {
        self.add_observer(observer)
    }

    pub fn add_arc_observer(&mut self, observer: Arc<dyn Observer<E>>) -> usize {
        self.add_observer(observer)
    }

    pub fn add_arc_mutex_observer(&mut self, observer: Arc<Mutex<dyn Observer<E>>>) -> usize {
        self.add_observer(observer)
    }

    pub fn add_arc_mutex_mut_observer(
        &mut self,
        observer: Arc<Mutex<dyn MutObserver<E>>>,
    ) -> usize {
        self.add_observer(observer)
    }

    pub fn add_arc_rwlock_observer(&mut self, observer: Arc<RwLock<dyn Observer<E>>>) -> usize {
        self.add_observer(observer)
    }

    pub fn add_arc_rwlock_mut_observer(
        &mut self,
        observer: Arc<RwLock<dyn MutObserver<E>>>,
    ) -> usize {
        self.add_observer(observer)
    }
}
