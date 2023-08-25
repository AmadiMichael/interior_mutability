
use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>
}


impl <T> Rc<T> {
    fn new(v: T) -> Self {
        let inner = Box::new(RcInner {value: v, refcount: Cell::new(1)});
        // SAFETY: box does not give us a null pointer
        Rc {inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner))}, _marker: PhantomData }
    }
}


impl <T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {inner: self.inner, _marker: PhantomData }
    }
}

impl <T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine
        &unsafe {self.inner.as_ref()}.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();

        if c == 1 {
            // SAFETY:  we are the only Rc letf, and we are being dropped therefore after us there will be no Rc's and no refernces to T.
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // SAFTEY: there are otehr rcs so don;t drop the box!
            inner.refcount.set(c - 1)
        }
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bad() {

    }
}