use crossbeam_queue::ArrayQueue;
    
#[test]
fn push_two_elements_succesful() {
    let q = ArrayQueue::new(2);

    assert_eq!(q.push('a'), Ok(()));
    assert_eq!(q.push('b'), Ok(()));
}

#[test]
fn push_overflowing_element() {
    let q = ArrayQueue::new(2);

    assert_eq!(q.push('a'), Ok(()));
    assert_eq!(q.push('b'), Ok(()));
    assert_eq!(q.push('c'), Err('c'));
}