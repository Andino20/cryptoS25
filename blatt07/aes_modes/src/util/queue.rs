use std::collections::VecDeque;

pub struct FixedQueue<T> {
    queue: VecDeque<T>,
    capacity: usize,
}

impl<T> FixedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        FixedQueue {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.queue.len() == self.capacity {
            self.queue.pop_front(); // Remove oldest item
        }
        self.queue.push_back(item); // Insert new item
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.queue.iter()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
