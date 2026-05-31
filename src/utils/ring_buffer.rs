#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,
    count: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); capacity],
            capacity,
            head: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.buffer[self.head] = value;
        self.head = (self.head + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    pub fn iter(&self) -> RingBufferIter<'_, T> {
        RingBufferIter {
            buffer: &self.buffer,
            capacity: self.capacity,
            head: self.head,
            count: self.count,
            pos: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn last(&self) -> Option<&T> {
        if self.count == 0 {
            return None;
        }
        let idx = if self.head == 0 { self.capacity - 1 } else { self.head - 1 };
        Some(&self.buffer[idx])
    }

    pub fn values(&self) -> Vec<T> {
        self.iter().collect()
    }
}

pub struct RingBufferIter<'a, T> {
    buffer: &'a [T],
    capacity: usize,
    head: usize,
    count: usize,
    pos: usize,
}

impl<'a, T: Clone> Iterator for RingBufferIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.count {
            return None;
        }
        let idx = if self.count < self.capacity {
            self.pos
        } else {
            (self.head + self.pos) % self.capacity
        };
        self.pos += 1;
        Some(self.buffer[idx].clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}
