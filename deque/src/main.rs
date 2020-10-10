use ::deque::circular_buffer::CircularBuffer;


fn main() {
    let mut queue = CircularBuffer::new(20);
    queue.push_right(1);
    queue.push_right(2);
    queue.push_right(3);
    queue.push_right(1);
    queue.push_right(2);
    queue.push_right(3);
    queue.push_left(4);
    queue.push_left(4);
    queue.push_left(4);
    queue.push_left(4);

    queue.pop_left();
    queue.pop_left();
    queue.pop_right();

    let mut i = 0;
    while !queue.is_empty() {
        println!("#{}: {}", i, queue.pop_left());
        i += 1;
    }

    println!("Hello, world!");
}
