#![feature(iter_next_chunk)]

fn main() {
    next_chunk();

    size_hint();

    count();

    last();

    println!("Hello, iterator!");
}

fn next_chunk() {
    let mut iter = "johnward".chars();

    assert_eq!(iter.next_chunk().unwrap(), ['j', 'o']);
    assert_eq!(iter.next_chunk().unwrap(), ['h', 'n', 'w', 'a', 'r']);
}

fn size_hint() {
    let a = [1, 2, 3];

    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());

    let _ = iter.next();

    assert_eq!((2, Some(2)), iter.size_hint());
}

// Consumes iterator to give you the count
fn count() {
    let a = [1, 2, 3];

    assert_eq!(a.iter().count(), 3);
}

// Consumes the iterator to give you the last value
fn last() {
    let a = [1, 2, 3];

    assert_eq!(a.iter().last(), Some(&3));
}

fn advance_by() {}
