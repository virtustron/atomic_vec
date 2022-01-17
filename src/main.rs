mod my_array_queue;

use crossbeam_queue::ArrayQueue;

use crate::my_array_queue::MyArrayQueue;



fn main() {
    print!("Hello, crossbeam!");

    let q1 = ArrayQueue::new(2);

    let q2 = MyArrayQueue::new(2);
    

}

#[cfg(test)]
mod test;
