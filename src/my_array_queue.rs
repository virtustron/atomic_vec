use core::mem::MaybeUninit;
use core::sync::atomic::{self, AtomicUsize, Ordering};


struct MySlot<T> {
    stamp: AtomicUsize,                     // position in queue to compair with "head" and "tail"
    value: UnsafeCell<MaybeUninit<T>>,      // value of element in queue
}



pub struct MyArrayQueue<T> {
    head: CachePadded<AtomicUsize>,     // "stamp": lower bits represent the index, 
                                        //          upper bits represent the lap.

    tail: CachePadded<AtomicUsize>,     // "stamp": lower bits represent the index, 
                                        //          upper bits represent the lap.

    buffer: *mut Slot<T>,               // holding slots
    
    cap: usize,                         // capacity of queue

    one_lap: usize,                     // ??? "stamp" with the value of `{ lap: 1, index: 0 }`.

    _marker: PhantomData<T>,            // ??? Indicates that dropping an `ArrayQueue<T>` may drop elements of type `T`.
}