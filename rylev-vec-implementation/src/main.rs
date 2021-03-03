use std::alloc;
use std::ptr::NonNull;

use alloc::dealloc;

// Allocate for 4 items first, since 1 or 2 is too low
const INITIAL_SIZE: usize = 4;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        // On first push, allocate first memory
        if self.capacity == 0 {
            assert_ne!(std::mem::size_of::<T>(), 0, "T must not be zero sized");
            let layout: alloc::Layout =
                alloc::Layout::array::<T>(INITIAL_SIZE).expect("Could not allocate");

            // SAFETY: the layout is hardcoded to be 4 * size_of<T>, T is not zero sized
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate, null ptr");

            // SAFETY: Memory for this item has been just allocated
            unsafe { ptr.as_ptr().write(item) };

            self.ptr = ptr;
            self.capacity = INITIAL_SIZE;
            self.len = 1;
        } else if self.len < self.capacity {
            // SAFETY: Ensure the offset in bytes does not overflow usize
            self.len
                .checked_mul(std::mem::size_of::<T>())
                .expect("offset overflow");
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::align_of::<T>();
            let new_size = self.capacity * 2;
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not allocate, null ptr");

                // SAFETY: Memory for this item has been just allocated
                ptr.as_ptr().add(self.len).write(item);

                self.ptr = ptr;
                self.capacity = self.capacity * 2;
                self.len += 1;
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // Compute the pointer to the item: .add() offsets by index
        // Then, de-ref the pointer and reference it again
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            // Call drop on all items of the vector
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));

            // de-allocate the vec memory in the heap
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::align_of::<T>();
            let layout = alloc::Layout::from_size_align_unchecked(size, align);
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initialize() {
        let vec = MyVec::<usize>::new();
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity, 0);
        assert_eq!(vec.len, 0);
    }

    #[test]
    fn push_items() {
        let mut vec = MyVec::<usize>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn standard_vector_works() {
        let mut vec = Vec::<usize>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
