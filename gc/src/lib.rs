mod trace;

pub use gc_derive::Trace;
pub use trace::Trace;

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Heap {
    all: HashSet<*const ()>,
    alive: HashSet<*const ()>,
    droppers: HashMap<*const (), Box<dyn Fn()>>,
}

impl std::fmt::Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for x in self.all.iter().cloned() {
            set.entry(&(x as usize));
        }

        set.finish()
    }
}

impl Heap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allocate<T: Trace + 'static>(&mut self, thing: T) -> Gc<T> {
        let ptr: *const T = Box::leak(Box::new(thing));
        let is_newly_inserted = self.all.insert(ptr as *const ());
        let dropper = move || {
            let _ = unsafe { Box::from_raw(ptr as *mut T) };
        };
        self.droppers.insert(ptr as *const (), Box::new(dropper));

        assert!(
            is_newly_inserted,
            "Heap claimed to already be aware of newly allocated pointer"
        );

        Gc { ptr }
    }

    /// SAFETY: The caller must ensure that all currently-held Gc pointers are rooted in the given root.
    pub unsafe fn collect_garbage<T: Trace>(&mut self, root: &T) {
        assert!(self.alive.is_empty());
        let mut m = Marker { heap: self };
        root.trace(&mut m);

        for dead in self.all.drain() {
            self.droppers.remove(&dead).unwrap()();
        }
        std::mem::swap(&mut self.alive, &mut self.all);
    }

    fn set_alive(&mut self, ptr: *const ()) {
        if self.all.remove(&ptr) {
            self.alive.insert(ptr);
        }
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        if !self.all.is_empty() {
            panic!("Heap leaked uncollected gc pointers");
        }
    }
}

pub struct Marker<'a> {
    heap: &'a mut Heap,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Gc<T: Trace> {
    ptr: *const T,
}

unsafe impl<T: Trace> Trace for Gc<T> {
    fn trace(&self, m: &mut Marker) {
        m.heap.set_alive(self.ptr as *const ());
    }
}

impl<T: Trace> std::ops::Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_gcptr_size() {
        assert_eq!(size_of::<Gc<i32>>(), size_of::<*const i32>());
    }
}
