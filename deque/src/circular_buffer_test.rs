use super::*;

#[test]
fn test_push_right() {
    let mut deque = CircularBuffer::new(5);
    let expected_buffer = [2, 3, 5, 7, 0];

    deque.push_right(2);
    deque.push_right(3);
    deque.push_right(5);
    deque.push_right(7);

    assert_eq!(deque.array[0..5], expected_buffer);
}

#[test]
fn test_push_left() {
    let mut deque = CircularBuffer::new(5);
    let expected_buffer = [2, 0, 7, 5, 3];

    deque.push_left(2);
    deque.push_left(3);
    deque.push_left(5);
    deque.push_left(7);

    assert_eq!(deque.array[0..5], expected_buffer);
}

#[test]
#[should_panic(expected = "Queue is full.")]
fn test_should_panic_when_push_right_on_full() {
    let mut deque = CircularBuffer::new(1);

    deque.push_right(2);
    deque.push_right(2);
}


#[test]
#[should_panic(expected = "Queue is full.")]
fn test_should_panic_when_push_left_on_full() {
    let mut deque = CircularBuffer::new(1);

    deque.push_left(2);
    deque.push_left(2);
}

#[test]
#[should_panic(expected = "Queue is empty.")]
fn test_should_panic_when_pop_right_on_empty() {
    let mut deque = CircularBuffer::new(5);

    deque.pop_right();
}


#[test]
#[should_panic(expected = "Queue is empty.")]
fn test_should_panic_when_pop_left_on_emptyl() {
    let mut deque = CircularBuffer::new(5);

    deque.pop_left();
}
