

use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}


// unsafe impl<T> Sync for Cell<T> {} 

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        // SAFETY: No one else is concurrnetly mutating self.vakue (because !sync)
        // SAFETY: We are not inavlidiating any references becaus wew never give any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T where T: Copy {
        // SAFETY: No one is modifying this value since only this thread can mutate (because !sync) 
        // and it is ececuting this function instead.
        unsafe { *self.value.get() }
    }
}


#[cfg(test)]
mod test {
    use super::Cell;


    #[test]
    fn bad() {
        // use std::sync::Arc;

        // let x = Arc::new(Cell::new(0));
        // let x1 = Arc::clone(&x);

        // let jh1 = std::thread::spawn(move || {
        //     for _ in 0..100000 {
        //         let x = x1.get();
        //         x1.set(x + 1);
        //     }
        // });

        // let x2 = Arc::clone(&x);
        // let jh2 = std::thread::spawn(move || {
        //     for _ in 0..100000 {
        //         let x = x2.get();
        //         x2.set(x + 1);
        //     }
        // });

        // jh1.join().unwrap();
        // jh2.join().unwrap();

        // assert_eq!(x.get(), 200000);
    }
}