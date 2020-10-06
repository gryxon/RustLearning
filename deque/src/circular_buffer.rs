const MAX_BUFFER_SIZE: usize = 10000;

pub struct CircularBuffer {
        capacity: usize,
        array: [Option<i32>; MAX_BUFFER_SIZE],
        first: usize,
        last: usize,
        length: usize,
    }


impl CircularBuffer{
    pub fn new(capacity: usize) -> CircularBuffer{
        CircularBuffer{
           capacity,
           array: [None; MAX_BUFFER_SIZE],
               first: 0,
               last: 0,
               length: 0,
            }
        }

        pub fn push_left(&mut self, element: i32) {
            self.push(true, CircularBuffer::next_left_index, element)
        }

        pub fn pop_left(&mut self) -> Option<i32> {
            self.pop(self.first, CircularBuffer::next_right_index)
        }

        pub fn push_right(&mut self, element: i32) {
            self.push(false, CircularBuffer::next_right_index, element)
        }

        pub fn pop_right(&mut self) -> Option<i32> {
            self.pop(self.last, CircularBuffer::next_left_index)
        }

        pub fn is_empty(&self) -> bool {
             self.length == 0
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
