use std::mem;
use std::ptr;
use std::alloc::dealloc;
use std::alloc::alloc_zeroed;
use std::alloc::realloc;

struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> MyVec<T> {
        MyVec {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn push(&mut self, item: T) {
        if self.len == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            let new_ptr = if self.cap == 0 {
                unsafe { alloc_zeroed(new_cap * mem::size_of::<T>()) as *mut T }
            } else {
                let old_ptr = self.ptr;
                unsafe {
                    realloc(
                        old_ptr as *mut u8,
                        self.cap * mem::size_of::<T>(),
                        new_cap * mem::size_of::<T>(),
                    ) as *mut T
                }
            };
            self.ptr = new_ptr;
            self.cap = new_cap;
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.add(self.len))) }
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            unsafe {
                dealloc(self.ptr as *mut u8, self.cap * mem::size_of::<T>());
            }
        }
    }
}
fn main() {
    let mut my_vec = MyVec::<i32>::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);

    if let Some(item) = my_vec.pop() {
        println!("Popped item: {}", item);
    }

    if let Some(value) = my_vec.get(0) {
        println!("Value at index 0: {}", value);
    }
}
