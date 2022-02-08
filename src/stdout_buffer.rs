use std::collections::VecDeque;

pub struct StdoutBuffer {
    buffer: FirstInLastOut<String>,
}

impl StdoutBuffer {
    pub fn new() -> StdoutBuffer {
        StdoutBuffer {
            buffer: FirstInLastOut::new()
        }
    }
}

struct FirstInLastOut<T> {
    queue: VecDeque<T>
}

impl<T> FirstInLastOut<T> {
    fn new() -> FirstInLastOut<T> {
        FirstInLastOut{
            queue: VecDeque::new(),
        }
    }

    fn with_capacity(capacity: usize) -> FirstInLastOut<T> {
        FirstInLastOut{
            queue: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, item: T) {
        self.queue.push_front(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_back()
    }
}