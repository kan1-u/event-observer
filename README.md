# Event Observer

Implementation of observer pattern by rust.

## Examples

- Immutable

```rust
use event_observer::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

enum MyEvent {
    A,
    B,
}

type MySubject = Subject<MyEvent>;

struct MyObserver {
    name: String,
}

impl Observer<MyEvent> for MyObserver {
    fn on_notify(&self, event: &MyEvent) {
        match event {
            MyEvent::A => println!("Hello {}", self.name),
            MyEvent::B => println!("Good bye {}", self.name),
        }
    }
}

let mut subject = MySubject::new();

let o1 = MyObserver { name: "Noah".to_string() };
let o2 = Rc::new(MyObserver { name: "Olivia".to_string() });
let o3 = Rc::new(RefCell::new(MyObserver { name: "Liam".to_string() }));
let o4 = Arc::new(MyObserver { name: "Emma".to_string() });
let o5 = Arc::new(Mutex::new(MyObserver { name: "Elijah".to_string() }));
let o6 = Arc::new(RwLock::new(MyObserver { name: "Ava".to_string() }));

subject.add_observer(o1);
subject.add_rc_observer(o2);
subject.add_rc_refcell_observer(o3);
subject.add_arc_observer(o4);
subject.add_arc_mutex_observer(o5);
subject.add_arc_rwlock_observer(o6);

subject.notify(&MyEvent::A);
subject.notify(&MyEvent::B);
```

- Mutable

```rust
use event_observer::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

enum MyEvent {
    X(usize),
    Y(usize),
}

type MySubject = Subject<MyEvent>;

struct MyState {
    x: usize,
    y: usize,
}

impl MutObserver<MyEvent> for MyState {
    fn on_notify(&mut self, event: &MyEvent) {
        match event {
            MyEvent::X(i) => self.x += i,
            MyEvent::Y(i) => self.y += i,
        }
    }
}

let mut subject = MySubject::new();

let o1 = Rc::new(RefCell::new(MyState { x: 0, y: 0 }));
let o2 = Arc::new(Mutex::new(MyState { x: 0, y: 0 }));
let o3 = Arc::new(RwLock::new(MyState { x: 0, y: 0 }));

subject.add_rc_refcell_mut_observer(o1.clone());
subject.add_arc_mutex_mut_observer(o2.clone());
subject.add_arc_rwlock_mut_observer(o3.clone());

subject.notify(&MyEvent::X(1));
subject.notify(&MyEvent::Y(2));

assert_eq!(o1.borrow().x, 1);
assert_eq!(o1.borrow().y, 2);
assert_eq!(o2.lock().unwrap().x, 1);
assert_eq!(o2.lock().unwrap().y, 2);
assert_eq!(o3.read().unwrap().x, 1);
assert_eq!(o3.read().unwrap().y, 2);
```
