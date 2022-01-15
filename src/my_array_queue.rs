use core::mem::MaybeUninit;
use core::sync::atomic::{self, AtomicUsize, Ordering};

use alloc::boxed::Box;


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

unsafe impl<T: Send> Sync for MyArrayQueue<T> {}
unsafe impl<T: Send> Send for MyArrayQueue<T> {}

impl<T> MyArrayQueue<T> {

    
    // Creates a new bounded queue with the given capacity.
    // Panics if the capacity is zero.
    pub fn new(cap: usize) -> MyArrayQueue<T> {
        assert!(cap > 0, "capacity must be non-zero");

        // Head is initialized to `{ lap: 0, index: 0 }`.
        // Tail is initialized to `{ lap: 0, index: 0 }`.
        let head = 0;
        let tail = 0;

        // Allocate a buffer of `cap` slots initialized with stamps.
        let buffer = {
            let boxed: Box<[Slot<T>]> = (0..cap)
                .map(|i| {
                    // Set the stamp to `{ lap: 0, index: i }`.
                    Slot {
                        stamp: AtomicUsize::new(i),
                        value: UnsafeCell::new(MaybeUninit::uninit()),
                    }
                })
                .collect();
            Box::into_raw(boxed) as *mut Slot<T>
        };

        // One lap is the smallest power of two greater than `cap`.
        let one_lap = (cap + 1).next_power_of_two();

        MyArrayQueue {
            buffer,
            cap,
            one_lap,
            head: CachePadded::new(AtomicUsize::new(head)),
            tail: CachePadded::new(AtomicUsize::new(tail)),
            _marker: PhantomData,
        }
    }

}