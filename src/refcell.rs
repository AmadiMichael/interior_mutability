use std::cell::UnsafeCell;
use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive
}


// unsafe impl<T> !Sync for RefCell<T>{}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared( 1));
                Some(Ref{refcell: self})
                // SAFETY: No exclusive references have been given out, since state would be exclusive
                // Some(unsafe {&*self.value.get()})
            }
            RefState::Shared(count) => {
                self.state.set(RefState::Shared(count + 1));
                Some(Ref{refcell: self})
                // SAFETY: No exclusive references have been given out, since state would be exclusive
                // Some(unsafe{ &*self.value.get() })
            }
            RefState::Exclusive => None
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state would be Shared or Exclusive
            Some(RefMut {refcell: self})
        } else {
            None
        }
    }
}


pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exclusive references have been given out.
        // once it is given out, state is set to shared so that no exclusive references are given out.
        // So de-referencing to a shared reference is fine
        unsafe {&*self.refcell.value.get()}
    }
}

impl <T> Drop for Ref<'_, T> {
     fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n -  1));
            }
        }
     }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}


impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: see safety for deref mut
        unsafe {&*self.refcell.value.get()}
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to Exclusive so that no future references are given out.
        // So we have an exclusive lease n the inner value, so mutably de-referencing is fine
        unsafe {&mut *self.refcell.value.get()}
    }
}

impl <T> Drop for RefMut<'_, T> {
     fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
     }
}

#[cfg(test)]
mod test {
    #[test]
    fn bad() {

    }
}
