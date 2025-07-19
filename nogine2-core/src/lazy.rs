use std::sync::RwLock;

use crate::crash;

// The validity of what I'm doing here is more than questionable but whatever.

/// Questionable struct that allows to create something the first time and clone it the rest of the times.
pub struct LazyCloner<T: Clone> {
    item: RwLock<Option<T>>,
    creator: fn() -> T,
}

impl<T: Clone> LazyCloner<T> {
    pub const fn new(creator: fn() -> T) -> Self {
        Self { item: RwLock::new(None), creator }
    }

    pub fn get(&self) -> T {
        match self.item.read() {
            Ok(x) => if let Some(x) = x.as_ref() { return x.clone(); },
            Err(_) => crash!("Couldn't access Lazy item!"),
        }

        match self.item.write() {
            Ok(mut x) => {
                let item = (self.creator)();
                let ret = item.clone();
                *x = Some(item);
                return ret;
            },
            Err(_) => crash!("Couldn't access Lazy item!"),
        }
    }
}
