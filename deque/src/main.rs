const MAX_BUFFER_SIZE: usize = 10000;

struct CircularBuffer {
    capacity: usize,
    array: [Option<i32>; MAX_BUFFER_SIZE],
    first: usize,
    last: usize,
    length: usize,
}


impl CircularBuffer{
    fn new(capacity: usize) -> CircularBuffer{
        CircularBuffer{
           capacity,
           array: [None; MAX_BUFFER_SIZE],
           first: 0,
           last: 0,
           length: 0,
        }
    }

    fn push_left(&mut self, element: i32) {
        self.push(true, CircularBuffer::next_left_index, element)
    }

    fn pop_left(&mut self) -> Option<i32> {
        self.pop(self.first, CircularBuffer::next_right_index)
    }

    fn push_right(&mut self, element: i32) {
        self.push(false, CircularBuffer::next_right_index, element)
    }

    fn pop_right(&mut self) -> Option<i32> {
        self.pop(self.last, CircularBuffer::next_left_index)
    }

    fn push(&mut self, use_first: bool, updater: fn(&CircularBuffer, usize) -> usize, element: i32){
        if !self.is_full(){
            let mut indicator = if use_first { self.first } else { self.last };
            if self.array[indicator] != None {
                if use_first{
                    self.first = updater(&self, self.first);
                    indicator = self.first;
                }
                else{
                    self.last = updater(&self, self.last);
                    indicator = self.last;
                }
            }
            self.array[indicator] = Some(element);
            self.length += 1;
        }
        // What to do?
    }

    fn pop(&mut self, indicator: usize, updater: fn(&CircularBuffer, usize) -> usize) -> Option<i32> {
        let result = self.array[indicator];
        match result {
            None => None,
            _ => {
                self.array[indicator] = None;
                if self.length != 1 {
                    if indicator == self.first{
                        self.first = updater(self, indicator);
                    }
                    else{
                        self.last = updater(self, indicator);
                    }
                }
                self.length -= 1;
                result
            },
        }
    }

    fn is_full(&self) -> bool {
         self.length == self.capacity
    }

    fn next_left_index(&self, current_index: usize) -> usize {
        if current_index == 0 { self.capacity - 1 } else { current_index - 1 }
    }

    fn next_right_index(&self, current_index: usize) -> usize {
        (current_index + 1) % self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deque_as_queue() {
        assert_eq!(false, true);
    }

    #[test]
    fn test_dequeu_as_stack() {
        assert_eq!(false, true);
    }

    #[test]
    fn test_push_from_two_sides() {
        assert_eq!(false, true);
    }

    #[test]
    fn test_pop_from_two_sides() {
        assert_eq!(false, true);
    }

    #[test]
    fn test_pop_from_empty() {
        assert_eq!(false, true);
    }

    #[test]
    fn test_deque_empty_again_and_pop_when_empty() {
        assert_eq!(false, true);
    }

}

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

    for i in queue.first..queue.first + queue.length {
        println!("#{}: {}", i - queue.first, queue.array[i%queue.capacity].unwrap());
    }

    println!("Hello, world!");
}
