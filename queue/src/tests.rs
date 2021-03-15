use super::*;

#[test]
fn test_queue_pop_works() {
    let mut q = Queue::new();
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('@');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('@'));
    assert_eq!(q.pop(), None);
}

#[test]
fn test_queue_push_works() {
    let mut q = Queue::new();
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
}

#[test] 
fn test_queue_is_empty_works() {
    let mut q = Queue::new();
    assert_eq!(q.is_empty(), true);
    q.push('1');
    assert_eq!(q.is_empty(), false);
}

#[test] 
fn test_queue_split_works() {
    let mut q = Queue::new();
    q.push('p');
    q.push('D');
    assert_eq!(q.pop(), Some('p'));
    q.push('X');

    let (older, younger) = q.split();

    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
