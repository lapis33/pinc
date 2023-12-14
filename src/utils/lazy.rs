use core::cell::{Cell, OnceCell};
use core::ops::{Deref, DerefMut};

pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}

impl<T, F> Lazy<T, F> {
    pub const fn new(f: F) -> Lazy<T, F> {
        Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(f)),
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cell.get_or_init(|| match self.init.take() {
            Some(f) => f(),
            None => unreachable!(),
        })
    }
}

impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell.get_or_init(|| match self.init.take() {
            Some(f) => f(),
            None => unreachable!(),
        });

        self.cell.get_mut().unwrap_or_else(|| unreachable!())
    }
}
